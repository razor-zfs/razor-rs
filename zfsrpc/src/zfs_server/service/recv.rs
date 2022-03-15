use tokio::io::AsyncWriteExt;
use tokio_pipe::pipe;
use tonic::Status;

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
    let mut expected_sequence = segment.sequence + 1;
    debug!(sequence = segment.sequence, "Receiving message");

    let (reader, mut writer) = pipe()?;
    let receiver = task::spawn_blocking(|| zfs::Zfs::receive(snapname, origin, false, reader));
    writer.write_all(&segment.buffer).await?;

    while let Some(segment) = input.message().await? {
        debug!(sequence = segment.sequence, "Receiving message");
        if expected_sequence == segment.sequence {
            expected_sequence = segment.sequence + 1;
        } else {
            let message = format!(
                "Message sequence mismatch: received {}, expected {}",
                segment.sequence, expected_sequence
            );
            return Err(Status::invalid_argument(message));
        }

        writer.write_all(&segment.buffer).await?;
    }

    receiver
        .await
        .map_err(join_to_status)?
        .map_err(zfs_to_status)?;

    Ok(response)
}
