use rweb::{get, reply, Reply};

use crate::state;

const DEFAULT_LISTEN_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 80);

pub(crate) async fn serve() {
    rweb::serve(zpool()).run(DEFAULT_LISTEN_ADDR).await
}

#[get("/zpool")]
fn zpool() -> impl Reply {
    let zpool = state::Zfs::pools();
    reply::json(&*zpool)
}

#[get("/zfs")]
fn zfs() -> impl Reply {
    let zfs = state::Zfs::datasets();
    reply::json(&*zfs)
}
