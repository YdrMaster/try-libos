#![no_std]
#![no_main]

use platform::{Platform, PlatformImpl};

#[no_mangle]
fn obj_main() {
    stdio::set_log_level(option_env!("LOG"));
    stdio::init(&Stdio);
    app::app_main();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    stdio::log::error!("{info}");
    PlatformImpl::shutdown(true);
    loop {}
}

struct Stdio;

impl stdio::Stdio for Stdio {
    #[inline]
    fn put_char(&self, c: u8) {
        PlatformImpl::console_putchar(c);
    }

    #[inline]
    fn put_str(&self, s: &str) {
        PlatformImpl::console_put_str(s);
    }

    #[inline]
    fn get_char(&self) -> u8 {
        PlatformImpl::console_getchar()
    }
}
