use std::backtrace;
use std::io::Write;
use std::sync::OnceLock;
use std::{fs::OpenOptions, str::FromStr};
thread_local! {
    static PANIC_COUNT : std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

#[derive(Debug)]
pub struct PaincConf {
    pub version: String,
    pub build_time: String,
    // 是否退出
    pub painc_exit: bool,
}

impl Default for PaincConf {
    fn default() -> Self {
        PaincConf {
            version: "0.0.1".to_string(),
            build_time: "2023-05-01".to_string(),
            painc_exit: true,
        }
    }
}

static VERSION_INFO: OnceLock<PaincConf> = OnceLock::new();
pub fn get_version() -> &'static PaincConf {
    VERSION_INFO.get().expect("version info get error")
}

pub fn set_panic_handler(v: PaincConf) {
    VERSION_INFO.set(v).expect("version info set error");

    std::panic::set_hook(Box::new(move |info| {
        PANIC_COUNT.with(|c| {
            let mut count = c.borrow_mut();
            *count += 1;
        });
        let panic_count = PANIC_COUNT.with(|c| *c.borrow());
        if panic_count > 1 {
            println!("panic happened more than once, exit immediatel y");
            std::process::exit(1);
        }

        let payload = info.payload();
        let payload_str: Option<&str> = if let Some(s) = payload.downcast_ref::<&str>() {
            Some(s)
        } else if let Some(s) = payload.downcast_ref::<String>() {
            Some(s)
        } else {
            None
        };

        let payload_str = payload_str.unwrap_or("<unknown panic info>");
        let location = info.location().unwrap();
        let thread = std::thread::current();
        let thread = thread.name().unwrap_or("<unnamed>");

        let tmp_path = std::env::temp_dir().join("panic.log");
        let candidate_path = vec![
            std::path::PathBuf::from_str("panic.log").ok(),
            Some(tmp_path),
        ];

        let mut file = None;
        let mut file_path = None;
        for path in candidate_path.iter().filter_map(|p| p.clone()) {
            file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path.clone())
                .ok();
            if file.is_some() {
                file_path = Some(path);
                break;
            }
        }

        // write str to stderr & file
        let write_err = |s: String| {
            let mut stderr = std::io::stderr();
            let content = format!("{}: {}", chrono::Local::now(), s);
            let _ = writeln!(stderr, "{}", content);
            if let Some(mut f) = file.as_ref() {
                let _ = writeln!(f, "{}", content);
            }
        };

        write_err(format!(
            "panic occurred:-----------------------------start-----------------------------"
        ));

        let info = get_version();
        write_err(format!("app version: {}", info.version));
        write_err(format!("app build date: {}", info.build_time));

        write_err(format!("os version: {}", std::env::consts::OS));
        write_err(format!("arch: {}", std::env::consts::ARCH));

        write_err(format!(
            "panic is recorded in: {}",
            file_path
                .and_then(|p| p.to_str().map(|x| x.to_string()))
                .unwrap_or("<no file>".to_string())
        ));
        write_err(format!("thread: {}", thread));
        write_err(format!("time: {}", chrono::Local::now()));
        write_err(format!("location: {}", location));
        write_err(format!("panic info: {}", payload_str));

        // backtrace is risky, so use it last
        let backtrace = backtrace::Backtrace::force_capture();
        write_err(format!("backtrace: {:#?}", backtrace));
        write_err(format!(
            "panic occurred:-----------------------------end-----------------------------\n\n"
        ));
        if info.painc_exit {
            std::process::exit(1);
        }
    }));
}
