#![no_std]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(linkage)]

pub use platform::Platform;
pub use SifiveU as PlatformImpl;

use sbi_rt::*;

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
    obj_main();
    system_reset(Shutdown, NoReason);
    unreachable!()
}

pub struct SifiveU;

impl platform::Platform for SifiveU {
    #[inline]
    fn console_getchar() -> u8 {
        #[allow(deprecated)]
        match legacy::console_getchar() as isize {
            -1 => unimplemented!("opensbi legacy::console_getchar is broken"),
            c => c as _,
        }
    }

    #[inline]
    fn console_putchar(c: u8) {
        #[allow(deprecated)]
        legacy::console_putchar(c as _);
    }

    #[inline]
    fn frequency() -> usize {
        0
    }

    #[inline]
    fn rdtime() -> usize {
        riscv::register::time::read()
    }

    #[inline]
    fn shutdown(error: bool) {
        if error {
            system_reset(Shutdown, SystemFailure);
        } else {
            system_reset(Shutdown, NoReason);
        }
    }
}
