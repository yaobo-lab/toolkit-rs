use super::DATETIME_FORMAT;
use chrono::{DateTime, TimeDelta, Utc};
use std::sync::OnceLock;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OsPlatform {
    Linux,
    MacOs,
    Windows,
    Unknown,
}

impl OsPlatform {
    pub fn get() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" | "apple" => Self::MacOs,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::Windows => "windows",
            Self::MacOs => "macos",
            Self::Unknown => "unknown",
        }
    }

    pub fn get_str() -> &'static str {
        Self::get().as_str()
    }
}

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub version: String,
    pub os_platform: OsPlatform,
    pub os_version: String,
    pub build_timestamp: DateTime<Utc>,
    pub git_commit_id: Option<String>,
    pub git_commit_short_id: Option<String>,
    pub git_commit_timestamp: Option<DateTime<Utc>>,
}

impl AppInfo {
    pub fn new(
        version: &str,
        build_timestamp: DateTime<Utc>,
        git_commit_id: Option<String>,
        git_commit_short_id: Option<String>,
        git_commit_timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        let os_info = os_info::get();
        let os_version = Some(os_info.version().to_string())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            version: version.to_string(),
            os_version,
            build_timestamp,
            git_commit_id,
            git_commit_short_id,
            git_commit_timestamp,
            os_platform: OsPlatform::get(),
        }
    }

    pub fn print_app_info(&self) -> String {
        let build_timestamp = self.build_timestamp.format(DATETIME_FORMAT).to_string();
        let git_commit_id = self
            .git_commit_id
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        let rows = [
            ("Version", self.version.as_str()),
            ("Platform", self.os_platform.as_str()),
            ("OS Version", self.os_version.as_str()),
            ("Build Time", &build_timestamp.as_str()),
            ("Git Commit id", &git_commit_id.as_str()),
        ];

        let label_width = 15;
        let value_width = rows
            .iter()
            .map(|(_, value)| value.len())
            .max()
            .unwrap_or(42)
            .max(42);
        let border = format!(
            "+-{:-<label_width$}-+-{:-<value_width$}-+",
            "",
            "",
            label_width = label_width,
            value_width = value_width
        );

        let mut output = String::new();
        output.push_str(&border);
        output.push('\n');
        output.push_str(&format!(
            "| {:^width$} |\n",
            "Application Information",
            width = label_width + value_width + 3
        ));
        output.push_str(&border);
        output.push('\n');

        for (label, value) in rows {
            output.push_str(&format!(
                "| {:<label_width$} | {:<value_width$} |\n",
                label,
                value,
                label_width = label_width,
                value_width = value_width
            ));
        }

        output.push_str(&border);
        output
    }
}

pub static APP_UP_TIME_INSTANCE: OnceLock<AppUpTime> = OnceLock::new();

// Application up time
pub struct AppUpTime {
    time: DateTime<Utc>,
}

pub fn set_app_up_time() {
    APP_UP_TIME_INSTANCE
        .set(AppUpTime::new())
        .unwrap_or_default();
}

impl AppUpTime {
    pub fn new() -> Self {
        Self { time: Utc::now() }
    }

    pub fn get_startup_time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn time_delta_since(&self) -> TimeDelta {
        let now = Utc::now();
        now.signed_duration_since(self.time)
    }
}
