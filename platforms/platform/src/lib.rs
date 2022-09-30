#![no_std]

pub trait Platform {
    fn console_getchar() -> u8;
    fn console_putchar(c: u8);
    fn shutdown(error: bool);
}
