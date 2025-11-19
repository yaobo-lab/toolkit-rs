use std::process;
use toolkit_rs::{
    logger::{self, LogConfig, LogStyle},
    painc::{set_panic_handler, PaincConf},
};

//cargo run  --example demo
fn main() {
    set_panic_handler(PaincConf::default());
    let mut cfg = LogConfig::default();
    cfg.style = LogStyle::Module;
    logger::setup(LogConfig::default()).unwrap_or_else(|e| {
        println!("log setup err:{}", e);
        process::exit(1);
    });
    log::debug!("this is a debug log..");
    log::info!("this is a info log..");
    log::warn!("this is a warn log..");
    log::error!("this is a error log..");
}
