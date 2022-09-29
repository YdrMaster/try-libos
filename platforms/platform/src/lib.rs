#![no_std]

pub trait Platform {
    fn console_getchar(&self) -> u8;
    fn console_putchar(&self, c: u8);
    fn shutdown(&self, error: bool);
}
