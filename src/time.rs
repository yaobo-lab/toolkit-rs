//全局时间格式
pub const DATETIME_FORMAT: &str = "%Y.%m.%d %H:%M:%S";
//获取本地时间
pub fn get_local_time() -> String {
    chrono::Local::now().format(DATETIME_FORMAT).to_string()
}
