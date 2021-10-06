#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points

use core::panic::PanicInfo;

#[panic_handler]    // This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // don't mangle the function name
pub extern "C" fn _start() -> ! {
    loop {}
}