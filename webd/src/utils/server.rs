use std::{net::SocketAddr, sync::atomic::AtomicBool};

use axum::Router;
use log::info;
use std::future::Future;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static EXTERNAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Runtime::new().unwrap()
});

pub fn appd_spawn<F>(f : F)
    where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    EXTERNAL_RUNTIME.spawn(f);
}


pub async fn run<IntoString>(name: IntoString, addr: String, router: Router) -> anyhow::Result<()> 
where IntoString: Into<String>
{
    let name = name.into();
    println!("{} running on: http://{}", name, addr);
    info!("{} running on: http://{}", name, addr);
    let listner = tokio::net::TcpListener::bind(addr.clone()).await?;
    axum::serve(
        listner,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
    Ok(())
}

pub static A_REBOOT_SIG: AtomicBool = AtomicBool::new(false);