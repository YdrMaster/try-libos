#![no_std]
#![no_main]

#[no_mangle]
fn obj_main() {
    stdio::init(&Stdio);
    app::app_main();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    stdio::println!("{info}");
    platform::shutdown(true);
    loop {}
}

struct Stdio;

impl stdio::Stdio for Stdio {
    #[inline]
    fn put_char(&self, c: u8) {
        platform::console_putchar(c);
    }
}
