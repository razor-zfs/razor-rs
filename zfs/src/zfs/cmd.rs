use std::process::Stdio;
use tokio::process::{Child, Command};

use super::*;

const ZFS: &str = "/usr/sbin/zfs";

impl Zfs {
    pub fn send_cmd<S, F>(source: S, from: Option<F>) -> Result<Child>
    where
        S: AsRef<str>,
        F: AsRef<str>,
    {
        let mut send = Command::new(ZFS);
        send.arg("send");
        if let Some(from) = from {
            send.args(&["-i", from.as_ref()]);
        }
        send.arg(source.as_ref())
            .stdout(Stdio::piped())
            .kill_on_drop(true);
        let child = send.spawn()?;
        Ok(child)
    }

    pub fn receive_cmd<S, O>(snapname: S, origin: Option<O>, force: bool) -> Result<Child>
    where
        S: AsRef<str>,
        O: AsRef<str>,
    {
        let mut recv = Command::new(ZFS);
        recv.arg("receive")
            .arg(snapname.as_ref())
            .stdin(Stdio::piped())
            .kill_on_drop(true);
        if force {
            recv.arg("-F");
        }
        if let Some(origin) = origin {
            recv.args(["-o", &format!("origin={}", origin.as_ref())]);
        }
        let child = recv.spawn()?;
        Ok(child)
    }
}
