#![no_std]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(linkage)]

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
    system_reset(RESET_TYPE_SHUTDOWN, RESET_REASON_NO_REASON);
    unreachable!()
}

#[inline]
pub fn console_putchar(c: u8) {
    #[allow(deprecated)]
    legacy::console_putchar(c as _);
}

#[inline]
pub fn shutdown(error: bool) {
    system_reset(
        RESET_TYPE_SHUTDOWN,
        if error {
            RESET_REASON_SYSTEM_FAILURE
        } else {
            RESET_REASON_NO_REASON
        },
    );
}
