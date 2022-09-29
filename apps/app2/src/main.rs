#![no_std]
#![no_main]

#[macro_use]
extern crate libos;

#[no_mangle]
fn app_main() {
    println!("Hello, this is app2!");
}
