use std::fmt::Display;

use log::info;

pub fn enable_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let println_panic_msg = |msg: &str| {
            println!("{}", msg);
            info!("{}", msg);
        };

        if let Some(location) = panic_info.location() {
            println_panic_msg(&format!(
                "panic occurred location in file '{}' at line {}",
                location.file(),
                location.line()
            ));
        }
        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            println_panic_msg(&format!("panic occurred payload: {}", payload));
        }
        println_panic_msg(&format!("panic occurred: {:?}", panic_info));
        default_hook(panic_info);
    }));
}

pub fn enable_full_backtrace(){
	std::env::set_var("RUST_BACKTRACE", "full");
}

#[derive(Debug)]
pub struct BasicInfo {
    pub cpu: usize,
    pub f_mem: u64,
    pub mem: u64,
    pub f_disk: u64,
    pub disk: u64,
}

pub fn get_runtime_hardware_info() -> BasicInfo {
    BasicInfo {
        cpu: num_cpus::get_physical(),
        mem: sys_info::mem_info().unwrap().total,
        disk: sys_info::disk_info().unwrap().total,
        f_mem: sys_info::mem_info().unwrap().avail,
        f_disk: sys_info::disk_info().unwrap().free,
    }
}


impl Display for BasicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MB: u64 = 1024;
        write!(
            f,
            "CPU: {}, MEM: {}/{}MB, DISK: {}/{}MB",
            self.cpu,
            self.mem / MB - self.f_mem / MB,
            self.mem / MB,
            self.disk / MB - self.f_disk / MB,
            self.disk / MB
        )
    }
}