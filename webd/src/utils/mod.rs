pub mod init;
pub mod server;

pub use init::*;
use once_cell::sync::OnceCell;
pub use server::*;


pub static VERSION: OnceCell<String> = OnceCell::new();
