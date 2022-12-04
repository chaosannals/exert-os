#![feature(core_intrinsics)]
#![feature(lang_items)]
#![no_std]
#![no_main]

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use x86_64::instructions::{hlt};
use vga::{ Cursor };

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
  let mut cursor = Cursor::new();
  for _ in 0..(80*25) {
    cursor.print(b" ");
  }
  cursor.to_begin();
  write!(cursor, "{}", info).unwrap();

  loop {
    hlt();
  }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }

#[no_mangle]
pub extern "C" fn _start() -> ! {
  panic!("help!");
}
