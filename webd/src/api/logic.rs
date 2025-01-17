use std::sync::{Mutex, RwLock};

use axum::Router;
use once_cell::sync::OnceCell;
use tokio::runtime::Handle;

use crate::{config::CONFIG, utils};
use appd_core;


pub static SERVER_HANDLE: OnceCell<RwLock<Mutex<Handle>>> = OnceCell::new();

pub fn core_api() -> Router {
    appd_core::get_api(CONFIG.app.config.clone())
}

pub fn new_runtime() -> anyhow::Result<tokio::runtime::Runtime> {
    let ncpu = num_cpus::get_physical();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("core")
        .worker_threads(ncpu)
        .build()?;
    Ok(rt)
}

pub fn core_server(addr: &str) -> anyhow::Result<()> {
    let rt = new_runtime()?;

    let handle: &tokio::runtime::Handle = rt.handle();
    SERVER_HANDLE.set(RwLock::new(Mutex::new(handle.clone()))).unwrap();
    handle.block_on(utils::run("cored", addr.to_owned(), core_api()))?;
    Ok(())
}