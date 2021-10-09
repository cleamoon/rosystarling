#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]    // This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // don't mangle the function name
pub extern "C" fn _start() -> ! {
    /*let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }*/

    vga_buffer::print_something();

    loop {}
}