#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kosdemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kosdemo::{println, print, init};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    // use core::fmt::Write;
    // vga::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga::WRITER.lock(), ",\nsome numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World");
    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     // 写入非法地址触发 缺页异常
    //     *(0xdeadbeef as *mut u64) = 42;

    //     // 把报错的虚拟地址用作指针
    //     let ptr = 0x204fe0 as *mut u32;
    //     let x = *ptr; // 读成功。
    //     println!("read x {:?}", x);
    //     *ptr = 42; // 写失败。
    // }

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Lv4 page table at {:?}", level_4_page_table.start_address());

    let level_4_table_pointer = 0xffff_ffff_ffff_f000 as *const u64;
    for i in 0..10 {
        let entry = unsafe { *level_4_table_pointer.offset(i) };
        println!("Entry {}: {:#x}", i, entry);
    }

    // 无限递归引发栈溢出
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    print!("the number is {}", 1234);
    // panic!("panic content !!!");

    #[cfg(test)]
    test_main();

    kosdemo::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kosdemo::hlt_loop();
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