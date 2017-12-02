#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]
//#! defines attribute of current module
//attributes are metadata like lints, compiler features, links to foreign library
//feature allows feature-gated attributes (not stable) compilers
#![feature(lang_items)]
//prevents automatic linking of std because it relies on OS
#![no_std]

//used for libc function like memcpy, memove, memset etc
extern crate rlibc;
extern crate spin;
extern crate volatile;

//Disables automatic name mangling
//Want to call rust_main from assembly
#[macro_use]
mod vga_buffer;
#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);
    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    //let buffer_ptr = (0xb8000 + 1988) as *mut _;
    //unsafe { *buffer_ptr = hello_colored };

    loop{}
}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
