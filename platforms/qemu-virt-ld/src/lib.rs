#![no_std]

pub const LINKER: &str = "\
OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = 0x80200000;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : ALIGN(8) {
        _bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    _end = ALIGN(8);
}";
