# toolkit-rs

一个轻量的 Rust 工具库，围绕以下几类常用能力做了封装：

- 时间格式化
- 文件日志初始化与滚动清理
- panic 捕获与落盘
- 基于 Tokio 的延迟退出 / 延迟重启辅助函数

仓库地址：<https://github.com/yaobo-lab/toolkit-rs>

## 功能概览

### `time`

始终可用，无需开启 feature。

- `get_local_time()`: 返回当前本地时间字符串
- `DATETIME_FORMAT`: 默认时间格式常量，格式为 `%Y.%m.%d %H:%M:%S`

### `logger` feature

启用后提供日志初始化能力，底层基于 `flexi_logger` + `log`。

- 支持日志级别配置
- 支持日志文件大小滚动
- 支持按天保留或按文件数量保留
- 支持同时输出到控制台
- 支持多种输出格式

可用格式：

- `LogStyle::Default`
- `LogStyle::Line`
- `LogStyle::Module`
- `LogStyle::Full`

核心入口：

- `logger::setup(LogConfig)`

### `panic_handle` feature

启用后提供 panic hook：

- 记录 panic 时间、线程、位置、版本信息
- 输出 backtrace
- 将内容写入 `panic.log` 或系统临时目录中的 `panic.log`
- 可配置 panic 后是否立即退出进程

核心入口：

- `painc::set_panic_handler(PaincConf)`

说明：模块名当前为 `painc`，这是仓库现有公开 API 的一部分，README 按实际代码说明。

### 其他辅助函数

库根模块还提供两个异步辅助函数：

- `after_app_exist(sec)`: 延迟退出当前进程
- `after_reboot(sec)`: Linux 下延迟执行系统重启，Windows 下仅输出警告

这两个函数内部使用 Tokio 任务调度。

## 安装

`Cargo.toml` 示例：

```toml
[dependencies]
toolkit-rs = { version = "1.0.22", features = ["logger", "panic_handle"] }
```

如果你只需要时间工具：

```toml
[dependencies]
toolkit-rs = "1.0.22"
```

## Feature Flags

```toml
[dependencies]
toolkit-rs = { version = "1.0.22", features = ["logger"] }
```

- `logger`: 启用日志模块
- `panic_handle`: 启用 panic 捕获模块

当前仓库更推荐至少启用 `logger` feature 使用；因为库中部分辅助函数会引用 `log` 宏。

## 快速开始

### 1. 时间工具

```rust
use toolkit_rs::get_local_time;

fn main() {
    println!("now: {}", get_local_time());
}
```

### 2. 初始化日志

```rust
use toolkit_rs::logger::{self, LogConfig, LogStyle};

fn main() {
    let mut cfg = LogConfig::default();
    cfg.level = 4;
    cfg.style = LogStyle::Module;
    cfg.size_mb = 10;
    cfg.keep_file_count = Some(5);
    cfg.console = true;

    logger::setup(cfg).expect("init logger failed");

    log::debug!("debug message");
    log::info!("info message");
    log::warn!("warn message");
    log::error!("error message");
}
```

### 3. 设置 panic 处理

```rust
use toolkit_rs::painc::{set_panic_handler, PaincConf};

fn main() {
    set_panic_handler(PaincConf {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_time: "2026-03-20".to_string(),
        painc_exit: true,
    });

    panic!("something went wrong");
}
```

### 4. 组合使用

```rust
use std::process;
use toolkit_rs::{
    logger::{self, LogConfig, LogStyle},
    painc::{set_panic_handler, PaincConf},
};

fn main() {
    set_panic_handler(PaincConf::default());

    let mut cfg = LogConfig::default();
    cfg.style = LogStyle::Module;
    cfg.size_mb = 1;
    cfg.keep_file_count = Some(2);

    logger::setup(cfg).unwrap_or_else(|e| {
        eprintln!("log setup err: {}", e);
        process::exit(1);
    });

    log::info!("toolkit-rs started");
}
```

## 运行示例

仓库内置示例：

```bash
cargo run --example demo --features panic_handle,logger
```

## API 导出

库对部分常用依赖做了 re-export，便于统一使用：

- `toolkit_rs::anyhow`
- `toolkit_rs::chrono`
- `toolkit_rs::serde`
- `toolkit_rs::AppResult`

## 开发与验证

已在当前仓库验证：

```bash
cargo test --features logger,panic_handle
cargo run --example demo --features panic_handle,logger
```

补充说明：

- 当前 `cargo test` 在不启用 feature 的情况下会编译失败
- `tests/testcase.rs` 目前仅包含基础占位测试

## 许可证

MIT License，见 [LICENSE](./LICENSE)。
