#![no_std]
#![no_main]

use platform::Platform;

static mut PLAT: Option<&'static dyn Platform> = None;

#[no_mangle]
fn obj_main(plat: &'static dyn platform::Platform) {
    stdio::set_log_level(option_env!("LOG"));
    unsafe { PLAT = Some(plat) };
    stdio::init(&Stdio);
    app::app_main();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    stdio::log::error!("{info}");
    unsafe { PLAT.unwrap().shutdown(true) };
    loop {}
}

struct Stdio;

impl stdio::Stdio for Stdio {
    #[inline]
    fn put_char(&self, c: u8) {
        unsafe { PLAT.unwrap().console_putchar(c) };
    }

    #[inline]
    fn get_char(&self) -> u8 {
        unsafe { PLAT.unwrap().console_getchar() }
    }
}
