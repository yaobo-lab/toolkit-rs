use std::process;
// use std::thread;
// use std::time::Duration;
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
    cfg.keep_file_count = Some(2);
    cfg.cleanup_sync = Some(false);
    logger::setup(cfg).unwrap_or_else(|e| {
        println!("log setup err:{}", e);
        process::exit(1);
    });

    for i in 0..100 {
        log::debug!("this is a debug log..{}", i);
        log::info!("this is a info log..{}", i);
        log::warn!("this is a warn log..{}", i);
        log::error!("this is a error log..{}", i);
    }

    // for _ in 0..10000 {
    //     log::debug!("this is a debug log..");
    //     log::info!("this is a info log..");
    //     log::warn!("this is a warn log..");
    //     log::error!("this is a error log..");
    // }
    //thread::sleep(Duration::from_secs(600));
}
