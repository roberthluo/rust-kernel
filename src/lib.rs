
//#! defines attribute of current module
//attributes are metadata like lints, compiler features, links to foreign library
//feature allows feature-gated attributes (not stable) compilers
#![feature(lang_items)]
//prevents automatic linking of std because it relies on OS
#![no_std]


//Disables automatic name mangling
//Want to call rust_main from assembly
#[no_mangle]
pub extern fn rust_main() {}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
