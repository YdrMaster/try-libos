#![no_std]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(linkage)]

pub use platform::Platform;
pub use Virt as PlatformImpl;

use sbi_rt::*;
use spin::{Mutex, Once};
use uart_16550::MmioSerialPort;

#[linkage = "weak"]
#[no_mangle]
fn obj_main() {
    panic!()
}

#[link_section = ".text.entry"]
#[no_mangle]
#[naked]
unsafe extern "C" fn _start() -> ! {
    const STACK_SIZE: usize = 4096;

    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        "la sp, {stack} + {stack_size}",
        "j  {main}",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym rust_main,
        options(noreturn),
    )
}

extern "C" fn rust_main() -> ! {
    UART0.call_once(|| Mutex::new(unsafe { MmioSerialPort::new(0x1000_0000) }));
    obj_main();
    system_reset(RESET_TYPE_SHUTDOWN, RESET_REASON_NO_REASON);
    unreachable!()
}

pub struct Virt;

static UART0: Once<Mutex<MmioSerialPort>> = Once::new();

impl platform::Platform for Virt {
    #[inline]
    fn console_getchar() -> u8 {
        UART0.wait().lock().receive()
    }

    #[inline]
    fn console_putchar(c: u8) {
        UART0.wait().lock().send(c)
    }

    #[inline]
    fn console_put_str(str: &str) {
        let mut uart = UART0.wait().lock();
        for c in str.bytes() {
            uart.send(c);
        }
    }

    #[inline]
    fn frequency() -> usize {
        12_500_000
    }

    #[inline]
    fn rdtime() -> usize {
        riscv::register::time::read()
    }

    #[inline]
    fn shutdown(error: bool) {
        system_reset(
            RESET_TYPE_SHUTDOWN,
            if error {
                RESET_REASON_SYSTEM_FAILURE
            } else {
                RESET_REASON_NO_REASON
            },
        );
    }
}
