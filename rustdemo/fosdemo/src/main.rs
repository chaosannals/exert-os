#![feature(core_intrinsics)]
#![feature(lang_items)]
// #![feature(asm)]
#![no_std]
#![no_main]

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use vga::{Color, Cursor};
use x86_64::instructions::hlt;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    let mut cursor = Cursor::new(Color::White, Color::Red);
    for _ in 0..(80 * 25) {
        cursor.print(b" ");
    }
    cursor.to_begin();
    write!(cursor, "{}", info).unwrap();

    loop {
        hlt();
    }
}

//
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
}

fn main() -> ! {
    panic!("help!");
}
