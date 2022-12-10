#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kosdemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kosdemo::{println, print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga::WRITER.lock(), ",\nsome numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World");
    print!("the number is {}", 1234);
    // panic!("panic content !!!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kosdemo::test_panic_handler(info)
}

#[cfg(test)]
use kosdemo::{serial_println, serial_print};

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}