use std::os::unix::prelude::AsRawFd;
use std::pin::Pin;

use tokio::io::{AsyncReadExt, BufReader};
use tokio_pipe::pipe;
use tokio_stream::Stream;

use super::*;

const DEFAULT_BUF_SIZE: usize = 1024 * 1024;

pub type SendStream = Pin<Box<dyn Stream<Item = Result<proto::SendSegment, tonic::Status>> + Send>>;

impl proto::SendRequest {
    pub async fn execute(self) -> ZfsRpcResult<SendStream> {
        let Self { from, source } = self;
        let from = if from.is_empty() { None } else { Some(from) };
        let name = source.clone();
        let (reader, writer) = pipe()?;
        let fd = writer.as_raw_fd();
        let buf_size = task::spawn_blocking(move || max_pipe_size(fd))
            .await
            .map_err(join_to_status)??;
        let mut reader = BufReader::with_capacity(buf_size, reader);
        let sender = task::spawn_blocking(|| zfs::Zfs::send(source, from, writer));

        let send_stream = async_stream::try_stream! {
            let mut sequence = 0;
            let mut _send_complete = false;
            loop {
                let mut buffer = Vec::with_capacity(buf_size);
                let count = reader.read_buf(&mut buffer).await?;
                if count > 0 {
                    let segment = proto::SendSegment {
                            name: name.clone(),
                            sequence,
                            buffer,
                        };
                    yield segment;
                } else {
                    break;
                }
                sequence += 1;
            }
            sender
                .await
                .map_err(join_to_status)?
                .map_err(zfs_to_status)?;
        };
        Ok(Response::new(Box::pin(send_stream)))
    }

    pub async fn execute_process(self) -> ZfsRpcResult<SendStream> {
        let Self { from, source } = self;
        let from = if from.is_empty() { None } else { Some(from) };
        let name = source.clone();

        let mut send = Zfs::send_cmd(source, from).map_err(zfs_to_status)?;
        let stdout = send
            .stdout
            .take()
            .ok_or_else(|| tonic::Status::internal("Failed to get stdout from 'zfs send'"))?;
        let mut reader = BufReader::with_capacity(DEFAULT_BUF_SIZE, stdout);

        let send_stream = async_stream::try_stream! {
            let mut sequence = 0;
            loop {
                let mut buffer = Vec::with_capacity(DEFAULT_BUF_SIZE);
                let count = reader.read_buf(&mut buffer).await?;
                if count > 0 {
                    let segment = proto::SendSegment {
                            name: name.clone(),
                            sequence,
                            buffer,
                        };
                    yield segment;
                } else {
                    break;
                }
                sequence += 1;
            }
            let status = send.wait().await?;
            if !status.success() {
                if let Some(code) = status.code() {
                    error!(code = code, "'zfs send` exit");
                } else {
                    error!("'zfs send` killed by signal");
                }

            }
        };
        Ok(Response::new(Box::pin(send_stream)))
    }
}
