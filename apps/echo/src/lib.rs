#![no_std]

use stdio::*;

pub fn app_main() {
    let mut buffer: [u8; 256] = [0u8; 256];
    let mut i = 0;
    loop {
        let print = match get_char() {
            b'\r' | b'\n' => {
                println!();
                true
            }
            8 => {
                if i > 0 {
                    i -= 1;
                    print!("\x08 \x08");
                }
                false
            }
            c => {
                buffer[i] = c;
                print!("{}", c as char);
                i += 1;
                if i == buffer.len() {
                    log::warn!("The line is too long!");
                    true
                } else {
                    false
                }
            }
        };
        if print {
            if let Ok(line) = core::str::from_utf8(&buffer[..i]) {
                println!("{line}");
            } else {
                log::warn!("The line is not utf-8");
            }
            i = 0;
        }
    }
}
