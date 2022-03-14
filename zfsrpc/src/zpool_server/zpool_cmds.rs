// FIXME: This file is a temporary hack to get zpool basic commands to work

use std::path::PathBuf;

use crate::zfsrpc_proto::tonic_zpoolrpc::{method, property, Method, Property};

use anyhow::Context;
use tokio::process::Command;

use tracing::{debug, error, trace};

const DISK_BY_ID_PATH: &str = "/dev/disk/by-id";

pub(crate) async fn create(
    name: &str,
    method: Option<Method>,
    disks: Vec<String>,
    properties: Vec<Property>,
) -> anyhow::Result<()> {
    let zspan = tracing::debug_span!("ztool");
    let _guard = zspan.enter();

    debug!(name, ?method, ?disks, ?properties);

    let output = Command::new("zpool")
        .arg("list")
        .output()
        .await
        .with_context(|| "failed to get zpool list")?;

    let out = std::str::from_utf8(output.stdout.as_slice())
        .with_context(|| "failed to get output of zpool list")?;
    if out.contains(name) {
        debug!("{} already exists", name);
    } else {
        let output = Command::new("zpool")
            .arg("import")
            .arg(name)
            .output()
            .await
            .with_context(|| "failed to exec zpool import command")?;

        if output.status.success() {
            debug!("{} was imported", name);
        } else {
            debug!("Creating zpool {}", name);

            let mut cmd = Command::new("zpool");
            cmd.arg("create").arg(name);

            match method.and_then(|m| m.method) {
                Some(method::Method::Raidz(_)) => {
                    cmd.arg("raidz");
                }
                Some(method::Method::Mirror(_)) => {
                    cmd.arg("mirror");
                }
                None => (),
            }

            cmd.args(disks);

            properties
                .into_iter()
                .filter_map(|p| p.property)
                .for_each(|p| match p {
                    property::Property::Ashift(ashift) => {
                        let arg = format!("ashift={}", ashift);
                        cmd.args(&["-o", &arg]);
                    }
                    property::Property::Mountpoint(mp) => {
                        let arg = format!("mountpoint={}", mp);
                        cmd.args(&["-O", &arg]);
                    }
                    property::Property::Cachefile(cachefile) => {
                        let arg = format!("cachefile={}", cachefile);
                        cmd.args(&["-o", &arg]);
                    }
                });

            debug!(?cmd);

            let output = cmd
                .output()
                .await
                .with_context(|| format!("failed to create zpool {}", name))?;

            if !output.status.success() {
                let msg = std::str::from_utf8(&output.stderr)?.to_string();
                error!(?msg);
                return Err(anyhow::anyhow!(msg));
            }
            debug!("zpool {} was created", name);
        }
    }

    Ok(())
}

pub(crate) async fn destroy(name: &str) -> anyhow::Result<()> {
    let output = Command::new("zpool")
        .arg("destroy")
        .arg(name)
        .output()
        .await
        .with_context(|| format!("Failed to destroy zpool {}", name))?;

    debug!(?output.status);

    assert!(output.status.success(), "failed to destroy zpool {} ", name);

    debug!("zpool {} was destroyed", name);

    Ok(())
}

pub(crate) fn get_ebs_path(ebs_id: String) -> anyhow::Result<String> {
    let ebs = PathBuf::from(ebs_id.clone());

    let path = enumerate()
        .context("Failed to enumerate")?
        .find(|dev| dev.file_name().map_or(false, |dev| *dev == ebs))
        .with_context(|| format!("Device not found for EBS {}", ebs_id))?;

    let path = path.to_string_lossy().to_string();

    Ok(path)
}

fn enumerate() -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    let devices = std::path::Path::new(DISK_BY_ID_PATH)
        .read_dir()?
        .inspect(|e| trace!("scanning entry {:?}", e))
        .filter_map(Result::ok)
        .map(|entry| entry.path());

    Ok(devices)
}
