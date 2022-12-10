#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
// use crate::{print, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga::WRITER.lock(), ",\nsome numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World");
    print!("the number is {}", 1234);
    panic!("panic content !!!");
    // loop {}
}

#[no_mangle]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}