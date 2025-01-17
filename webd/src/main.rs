use std::{path::PathBuf, process::exit};

use webd::{self, api, config::CONFIG, utils::{self, VERSION}};
use clap::Parser;
use log::info;
use shadow_rs::shadow;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::util::SubscriberInitExt;


#[derive(Debug, Parser)]
#[command(version, about = "miniapp management", author = "kylin")]
struct Args {
    #[clap(short, long, default_value = "0.0.0.0:8000")]
    pub addr: String,

    #[clap(short, long, default_value = "config/webd.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Dump,
}

fn main() -> anyhow::Result<()> {
    let _guard = boot()?;

    if let Some(addr) = &CONFIG.admin_addr {
        utils::appd_spawn(utils::run("admin", addr.clone(), api::admin_api()));
    }

    api::core_server(&CONFIG.addr)?;

    Ok(())
}

fn boot() -> anyhow::Result<WorkerGuard> {
    let args = Args::parse();
    subcommand(&args);
    println!("{:?}", args);

    utils::enable_panic_hook();
    utils::enable_full_backtrace();

    // set config
    let _ = webd::config::CONFIG_PATH.set(args.config);
    let cfg = CONFIG.clone();
    println!("{:?}", cfg);

    // get build info
    shadow!(build);
    VERSION.set(build::VERSION.into()).unwrap();

    // init logger
    let lo = cfg.log.clone();
    let file_appender = tracing_appender::rolling::daily("logs", lo.path.clone());
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(lo.level)
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_line_number(true)
        .finish()
        .init();

    info!("{}", build::VERSION);
    info!("{:?}", cfg);

    Ok(guard)
}

fn subcommand(args: &Args) {
    if args.command.is_none() {
        return;
    }
    match args.command.as_ref().unwrap() {
        Command::Dump => {
            let c = include_str!("../config/webd.toml");
            println!("{}", c);
            exit(0);
        }
    }
}
