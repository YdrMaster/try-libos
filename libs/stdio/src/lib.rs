//! 提供 `print!`、`println!` 和 `log::Log`。

#![no_std]
#![deny(warnings, missing_docs)]

use core::{
    fmt::{Arguments, Write},
    str::FromStr,
};
use spin::Once;

/// 向用户提供 `log`。
pub extern crate log;

/// 这个接口定义了向控制台“输出”这件事。
pub trait Stdio: Sync {
    /// 向控制台放置一个字符。
    fn put_char(&self, c: u8);

    /// 向控制台放置一个字符串。
    ///
    /// 如果使用了锁，覆盖这个实现以免反复获取和释放锁。
    #[inline]
    fn put_str(&self, s: &str) {
        for c in s.bytes() {
            self.put_char(c);
        }
    }

    /// 从控制台读取一个字符。
    fn get_char(&self) -> u8;
}

/// 库找到输出的方法：保存一个对象引用，这是一种单例。
static CONSOLE: Once<&'static dyn Stdio> = Once::new();

/// 用户调用这个函数设置输出的方法。
pub fn init(console: &'static dyn Stdio) {
    CONSOLE.call_once(|| console);
    log::set_logger(&Logger).unwrap();
}

/// 根据环境变量设置日志级别。
pub fn set_log_level(env: Option<&str>) {
    use log::LevelFilter as Lv;
    log::set_max_level(env.and_then(|s| Lv::from_str(s).ok()).unwrap_or(Lv::Trace));
}

/// 打印一些测试信息。
pub fn test_log() {
    println!("[PRINT] LOG TEST >> Hello, world!");
    log::trace!("LOG TEST >> Hello, world!");
    log::debug!("LOG TEST >> Hello, world!");
    log::info!("LOG TEST >> Hello, world!");
    log::warn!("LOG TEST >> Hello, world!");
    log::error!("LOG TEST >> Hello, world!");
    println!();
}

/// 打印。
///
/// 给宏用的，用户不会直接调它。
#[doc(hidden)]
#[inline]
pub fn _print(args: Arguments) {
    Logger.write_fmt(args).unwrap();
}

/// 读取。
#[inline]
pub fn get_char() -> u8 {
    CONSOLE.wait().get_char()
}

/// 格式化打印。
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
    }
}

/// 格式化打印并换行。
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::_print(core::format_args!($($arg)*));
        $crate::println!();
    }}
}

struct Logger;

impl Write for Logger {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let _ = CONSOLE.get().unwrap().put_str(s);
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        use log::Level::*;
        let color_code: u8 = match record.level() {
            Error => 31,
            Warn => 93,
            Info => 34,
            Debug => 32,
            Trace => 90,
        };
        println!(
            "\x1b[{color_code}m[{:>5}] {}\x1b[0m",
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {}
}
