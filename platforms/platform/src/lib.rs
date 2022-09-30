#![no_std]

pub trait Platform {
    fn console_getchar() -> u8;
    fn console_putchar(c: u8);
    #[inline]
    fn console_put_str(str: &str) {
        for c in str.bytes() {
            Self::console_putchar(c);
        }
    }
    fn frequency() -> usize;
    fn rdtime() -> usize;
    fn shutdown(error: bool);
}
