// FIXME: This file is a temporary hack to get zpool basic commands to work

use crate::zfsrpc_proto::tonic_zpoolrpc::{method, property, Method, Property};

use tokio::process::Command;

use tracing::{debug, error};

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
        .expect("failed to get zpool list");

    let out = std::str::from_utf8(output.stdout.as_slice()).expect("failed to get output");
    if out.contains(name) {
        debug!("{} already exists", name);
    } else {
        let output = Command::new("zpool")
            .arg("import")
            .arg(name)
            .output()
            .await
            .expect("failed to exec zpool import command");

        if output.status.success() {
            debug!("{} was imported", name);
        } else {
            debug!("Creating zpool {}", name);

            let mut cmd = Command::new("zpool");
            cmd.arg("create").arg(name);

            if let Some(method) = method {
                match method.method {
                    Some(method::Method::Raidz(_)) => {
                        cmd.arg("raidz");
                    }
                    Some(method::Method::Mirror(_)) => {
                        cmd.arg("mirror");
                    }
                    None => unreachable!(),
                }
            };

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
                .unwrap_or_else(|_| panic!("zpool create {} failed", name));

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
        .unwrap_or_else(|_| panic!("failed to destroy zpool {}", name));

    debug!(?output.status);

    assert!(output.status.success(), "failed to destroy zpool {} ", name);

    debug!("zpool {} was destroyed", name);

    Ok(())
}
