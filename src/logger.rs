use anyhow::{anyhow, Result};
use ftlog::appender::{file::Period, ChainAppenders, FileAppender};
use ftlog::FtLogFormat;
use log::{Level, LevelFilter, Record};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::Path;
use std::sync::atomic::{AtomicU8, Ordering};
use time::Duration;

static LOG_STYLE: AtomicU8 = AtomicU8::new(0);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    //等级
    pub level: u8,
    //大小
    pub size: usize,
    //是否打印到控制台
    pub console: bool,
    // 日志输出文件
    pub file: String,
    // 日志保留文件数
    pub key_num: i64,
    // bounded
    pub bounded: Option<usize>,
    // 过滤日志模块
    pub filters: Option<Vec<String>>,
    // simple,line,module,full
    pub style: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: 4,
            size: 3 * 1024 * 1024,
            console: true,
            file: "./logs/app.log".to_string(),
            key_num: 3,
            filters: None,
            bounded: Some(1_000),
            style: Some("line".to_string()),
        }
    }
}

#[allow(dead_code)]
struct Msg {
    level: Level,
    thread: Option<String>,
    file: Option<&'static str>,
    line: Option<u32>,
    args: String,
    module_path: Option<&'static str>,
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //所有出错都打印 行号，文件路径
        if self.level == Level::Warn || self.level == Level::Error {
            f.write_str(&format!(
                "{}:{} [{}] {}",
                self.file.unwrap_or(""),
                self.line.unwrap_or(0),
                self.level,
                self.args
            ))
        } else {
            let s = LOG_STYLE.load(Ordering::Relaxed);
            match s {
                //line
                1 => f.write_str(&format!(
                    "{}:{} [{}] {}",
                    self.file.unwrap_or(""),
                    self.line.unwrap_or(0),
                    self.level,
                    self.args
                )),
                //module
                2 => f.write_str(&format!(
                    "{} {}:{} [{}] {}",
                    self.module_path.unwrap_or(""),
                    self.file.unwrap_or(""),
                    self.line.unwrap_or(0),
                    self.level,
                    self.args
                )),
                //full
                3 => f.write_str(&format!(
                    "{}:{}||{}:{} [{}] {}",
                    self.thread.as_ref().map(|x| x.as_str()).unwrap_or(""),
                    self.module_path.unwrap_or(""),
                    self.file.unwrap_or(""),
                    self.line.unwrap_or(0),
                    self.level,
                    self.args
                )),
                //none
                _ => f.write_str(&format!(
                    "{}:{} [{}] {}",
                    self.file.unwrap_or(""),
                    self.line.unwrap_or(0),
                    self.level,
                    self.args
                )),
            }
        }
    }
}
struct MyFormatter;

impl FtLogFormat for MyFormatter {
    fn msg(&self, record: &Record) -> Box<dyn Send + Sync + std::fmt::Display> {
        Box::new(Msg {
            level: record.level(),
            thread: std::thread::current().name().map(|n| n.to_string()),
            file: record.file_static(),
            line: record.line(),
            args: format!("{}", record.args()),
            module_path: record.module_path_static(),
        })
    }
}

pub fn string_to_level(level: &str) -> u8 {
    let v = match level {
        "trace" | "TRACE" => 5,
        "debug" | "DEBUG" => 4,
        "info" | "INFO" => 3,
        "warn" | "WARN" => 2,
        "error" | "ERROR" => 1,
        _ => 4,
    };
    v
}

pub fn setup(cfg: LogConfig) -> Result<()> {
    let level = match cfg.level {
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    };

    //日志风格
    let s: &str = &cfg.style.unwrap_or_default();
    let style: u8 = match s {
        "line" => 1,
        "module" => 2,
        "full" => 3,
        _ => 0,
    };
    LOG_STYLE.store(style, Ordering::Relaxed);

    let time_format = time::format_description::parse_owned::<1>(
        "[year].[month].[day] [hour]:[minute]:[second].[subsecond digits:2]",
    )
    .unwrap();

    let mut dir_str = "".to_string();
    let dir = Path::new(cfg.file.as_str());
    if let Some(p) = dir.parent() {
        dir_str = format!("{}", p.display());
        std::fs::create_dir_all(p)?;
    }

    let filter_log = if !dir_str.is_empty() {
        format!("{}/filters.log", dir_str)
    } else {
        format!("./filters.log")
    };

    let log_cfg = FileAppender::builder()
        .path(cfg.file.as_str())
        .rotate(Period::Day)
        .expire(Duration::days(cfg.key_num))
        .build();

    // 打印到不同的渠道
    let chains = if cfg.console {
        ChainAppenders::new(vec![Box::new(log_cfg), Box::new(std::io::stdout())])
    } else {
        ChainAppenders::new(vec![Box::new(log_cfg)])
    };

    let mut b = ftlog::Builder::new().format(MyFormatter);
    if let Some(v) = cfg.bounded {
        b = b.bounded(v, false);
    } else {
        b = b.unbounded();
    }

    let mut b = b
        .time_format(time_format)
        .max_log_level(level)
        .root(chains)
        .appender("ftlog-appender", FileAppender::new(filter_log));

    if let Some(items) = cfg.filters {
        for module in items {
            let static_module: &'static str = Box::leak(module.into_boxed_str());
            b = b.filter(static_module, "ftlog-appender", LevelFilter::Warn);
        }
    }

    //
    // b = b.filter("ureq", "ftlog-appender", LevelFilter::Warn);
    // b = b.filter("ureq_proto::client", "ftlog-appender", LevelFilter::Warn);
    // b = b.filter("ureq::unversioned", "ftlog-appender", LevelFilter::Warn);

    // ftlog::appender: 需要过滤的模块路径（ftlog::appender）
    // 指定使用的日志输出器名称（ftlog-appender）
    // 指定日志级别
    // .filter("app::startup", "ftlog-startup", LevelFilter::Warn)
    // .appender("ftlog-startup", FileAppender::new("./logs/startup.log"))
    b.try_init()
        .map_err(|e| anyhow!("logger init failed:{}", e.to_string()))?;

    Ok(())
}
