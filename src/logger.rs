use anyhow::Result;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, LogSpecification, Logger, Naming,
    Record,
};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
const TIME_FORMAT: &str = "%Y.%m.%d %H:%M:%S%.3f";
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogStyle {
    Default,
    Line,
    Module,
    Full,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    //等级
    #[serde(default = "default_level")]
    pub level: u8,
    //大小 MB
    #[serde(default = "default_usize")]
    pub size_mb: u64,
    //是否打印到控制台
    #[serde(default = "default_console")]
    pub console: bool,
    // 日志输出目录
    #[serde(default = "default_log_dir")]
    pub dir: String,
    // 日志保留文件数
    #[serde(default = "default_keep_day")]
    pub keep_day: usize,
    // 过滤日志模块
    pub filters: Option<Vec<String>>,
    pub style: LogStyle,
}

fn default_usize() -> u64 {
    3
}
fn default_console() -> bool {
    true
}
fn default_keep_day() -> usize {
    3
}
fn default_level() -> u8 {
    4
}
fn default_log_dir() -> String {
    "./logs".to_string()
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            size_mb: default_usize(),
            console: default_console(),
            dir: default_log_dir(),
            keep_day: default_keep_day(),
            filters: None,
            style: LogStyle::Default,
        }
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

pub fn _default(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    if record.level() == LevelFilter::Error || record.level() == LevelFilter::Warn {
        write!(
            w,
            "{} [{}] {}:{}: ",
            now.format(TIME_FORMAT),
            record.level(),
            record.file().unwrap_or("<unnamed>"),
            record.line().unwrap_or(0),
        )?;
    } else {
        write!(w, "{} [{}]: ", now.format(TIME_FORMAT), record.level(),)?;
    }
    write!(w, "{}", &record.args())
}

pub fn _line(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}] {}:{}: ",
        now.format(TIME_FORMAT),
        record.level(),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;
    write!(w, "{}", &record.args())
}

pub fn _module(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}] [{}]: ",
        now.format(TIME_FORMAT),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
    )?;
    write!(w, "{}", &record.args())
}

pub fn _full(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}] [{}] {}:{}: ",
        now.format(TIME_FORMAT),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;
    write!(w, "{}", &record.args())
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

    let mut log_spec = LogSpecification::builder();
    log_spec.default(level);
    if let Some(items) = &cfg.filters {
        for m in items {
            log_spec.module(m, LevelFilter::Warn);
        }
    }

    let hand = Logger::with(log_spec.build());

    //日志风格
    let hand = match cfg.style {
        LogStyle::Line => hand.format(_line),
        LogStyle::Module => hand.format(_module),
        LogStyle::Full => hand.format(_full),
        _ => hand.format(_default),
    };

    let mut hand = hand
        .rotate(
            Criterion::AgeOrSize(Age::Day, cfg.size_mb * 1024 * 1024),
            Naming::TimestampsCustomFormat {
                current_infix: None,
                format: "%Y.%m.%d",
            },
            Cleanup::KeepForDays(cfg.keep_day),
        )
        .log_to_file(FileSpec::default().directory(cfg.dir).basename(""));

    if cfg.console {
        let d = match cfg.level {
            1 => Duplicate::Error,
            2 => Duplicate::Warn,
            3 => Duplicate::Info,
            4 => Duplicate::Debug,
            5 => Duplicate::Trace,
            _ => Duplicate::Debug,
        };
        hand = hand.duplicate_to_stdout(d)
    }
    hand.start()?;
    Ok(())
}
