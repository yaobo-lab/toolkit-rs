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
    cfg.size_mb = 1;
    cfg.keep_day = 1;
    logger::setup(cfg).unwrap_or_else(|e| {
        println!("log setup err:{}", e);
        process::exit(1);
    });

    for _ in 0..100 {
        log::debug!("this is a debug log..");
        log::info!("this is a info log..");
        log::warn!("this is a warn log..");
        log::error!("this is a error log..");
    }

    // for _ in 0..10000 {
    //     log::debug!("this is a debug log..");
    //     log::info!("this is a info log..");
    //     log::warn!("this is a warn log..");
    //     log::error!("this is a error log..");
    // }
}
