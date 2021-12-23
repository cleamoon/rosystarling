#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rosystarling::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rosystarling::println;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // don't mangle the function name
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]    // This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info); 
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rosystarling::test_panic_handler(info)
}
