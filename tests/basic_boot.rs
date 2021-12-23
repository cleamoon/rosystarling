#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rosystarling::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rosystarling::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

/*fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}*/

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rosystarling::test_panic_handler(_info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}