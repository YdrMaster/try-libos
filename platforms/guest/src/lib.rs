#![feature(linkage)]

#[no_mangle]
fn main() {
    app_main()
}

#[linkage = "weak"]
#[no_mangle]
fn app_main() {
    panic!()
}
