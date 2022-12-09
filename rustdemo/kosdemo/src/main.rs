#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga::WRITER.lock(), ",\nsome numbers: {} {}", 42, 1.337).unwrap();
    loop {}
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}