use rweb::{get, http, reply, Filter, Rejection, Reply};

use crate::state;

const DEFAULT_LISTEN_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 80);

pub(crate) async fn serve() {
    rweb::serve(zpool().or(zfs()))
        .run(DEFAULT_LISTEN_ADDR)
        .await
}

#[get("/zpool")]
async fn zpool() -> Result<impl Reply, Rejection> {
    let state = state::Zfs::get();
    let state = state.lock().await;
    Ok(reply::json(state.pools()))
}

#[get("/zfs")]
async fn zfs() -> Result<impl Reply, Rejection> {
    let state = state::Zfs::get();
    let state = state.lock().await;
    Ok(reply::json(state.datasets()))
}

#[get("/refresh")]
async fn refresh() -> Result<impl Reply, Rejection> {
    let state = state::Zfs::get();
    let mut state = state.lock().await;

    if state.load().await.is_ok() {
        Ok(http::StatusCode::OK)
    } else {
        Ok(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
