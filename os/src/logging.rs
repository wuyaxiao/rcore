/*！

本模块利用 log crate 为你提供了日志功能，使用方式见 main.rs.

*/

use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        //"\x1b[31mhello world\x1b[0m"
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

pub fn init() {
//定义了一个静态变量 LOGGER，其类型为 SimpleLogger。静态变量在程序运行期间只会被初始化一次，并在整个程序的生命周期内保持不变。
    static LOGGER: SimpleLogger = SimpleLogger;
    //将 LOGGER 实例作为日志记录器设置为全局默认的日志记录器。unwrap() 方法用于处理可能的错误情况，如果设置日志记录器失败会导致程序崩溃。
    log::set_logger(&LOGGER).unwrap();
    //这行代码设置日志系统的最大日志级别。根据环境变量 "LOG" 的值，选择日志级别，并使用 match 匹配语句进行条件判断。
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}
