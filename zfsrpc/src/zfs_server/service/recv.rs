// use std::pin::Pin;

use tokio::io::AsyncWriteExt;
use tokio_pipe::pipe;
use tonic::Code;
use tonic::Status;
// use tokio_stream::Stream;
// use tokio_util::io::ReaderStream;

use super::*;

pub async fn recv(mut input: tonic::Streaming<proto::SendSegment>) -> ZfsRpcResult<proto::Empty> {
    let origin: Option<String> = None;
    let response = Response::new(proto::Empty {});
    let segment = if let Some(segment) = input.message().await? {
        segment
    } else {
        return Ok(response);
    };

    let snapname = segment.name;
    let mut sequence = segment.sequence;
    debug!("Receiving {sequence}");

    let (reader, mut writer) = pipe()?;
    let receiver = task::spawn_blocking(|| zfs::Zfs::receive(snapname, origin, false, reader));
    writer.write_all(&segment.buffer).await?;

    while let Some(segment) = input.message().await? {
        if sequence + 1 != segment.sequence {
            let cur_sequence = segment.sequence;
            let message = format!("Receiving {cur_sequence} while previous was {sequence:?}");
            return Err(Status::new(Code::InvalidArgument, message));
        }

        sequence = segment.sequence;
        debug!("Receiving {sequence:?}");
        writer.write_all(&segment.buffer).await?;
    }

    receiver
        .await
        .map_err(join_to_status)?
        .map_err(zfs_to_status)?;

    Ok(response)
}

// pub type SendStream = Pin<Box<dyn Stream<Item = Result<proto::SendSegment, tonic::Status>> + Send>>;

// impl proto::SendRequest {
//     pub async fn execute(self) -> ZfsRpcResult<SendStream> {
//         let Self { from, source } = self;
//         let from = if from.is_empty() { None } else { Some(from) };
//         let name = source.clone();
//         let (mut reader, writer) = pipe()?;
//         let sender = task::spawn_blocking(|| zfs::Zfs::send(source, from, writer));

//         let send_stream = async_stream::try_stream! {
//             let mut sequence = 0;
//             let mut _send_complete = false;
//             loop {
//                 let mut buffer = [0; 128 * 1024];
//                 let count = reader.read(&mut buffer).await?;
//                 if count > 0 {
//                     let segment = proto::SendSegment {
//                             name: name.clone(),
//                             sequence,
//                             buffer: buffer[0..count].to_vec(),
//                         };
//                     yield segment;
//                 } else {
//                     break;
//                 }
//                 sequence += 1;
//             }
//             sender
//                 .await
//                 .map_err(join_to_status)?
//                 .map_err(zfs_to_status)?;
//         };
//         Ok(Response::new(Box::pin(send_stream)))
//     }
// }
