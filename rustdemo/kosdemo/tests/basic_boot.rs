#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kosdemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// fn test_runner(_tests: &[&dyn Fn()]) {
//     unimplemented!();
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kosdemo::test_panic_handler(info)
}

use kosdemo::{println, serial_print, serial_println};

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}