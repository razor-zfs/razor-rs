use std::pin::Pin;

use tokio::io::AsyncReadExt;
use tokio_pipe::pipe;
use tokio_stream::Stream;
// use tokio_util::io::ReaderStream;

use super::*;

pub type SendStream = Pin<Box<dyn Stream<Item = Result<proto::SendSegment, tonic::Status>> + Send>>;

impl proto::SendRequest {
    pub async fn exec(self) -> ZfsRpcResult<SendStream> {
        let Self { from, source } = self;
        let from = if from.is_empty() { None } else { Some(from) };
        let name = source.clone();
        let (mut reader, writer) = pipe()?;
        let sender = task::spawn_blocking(|| zfs::Zfs::send(source, from, writer));

        let send_stream = async_stream::try_stream! {
            let mut sequence = 0;
            let mut _send_complete = false;
            loop {
                let mut buffer = Vec::with_capacity(128 * 1024);
                let count = reader.read(&mut buffer).await?;
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
}
