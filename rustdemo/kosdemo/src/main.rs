#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kosdemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use kosdemo::{println, print, init, memory};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
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

    // 启用分页机制后，直接通过物理地址读取失败。（因为此时寻址都采用分页机制，物理地址被当作虚拟地址拿去寻址了）
    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Lv4 page table at {:?}", level_4_page_table.start_address());
    // let level_4_table_pointer = 0xffff_ffff_ffff_f000 as *const u64;
    // for i in 0..10 {
    //     let entry = unsafe { *level_4_table_pointer.offset(i) };
    //     println!("Entry {}: {:#x}", i, entry);
    // }

    // 显示 L4 页表
    use kosdemo::memory::active_level_4_table;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let l4_table = unsafe {
        active_level_4_table(phys_mem_offset)
    };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}",i, entry);
        }
    }

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);

        // use kosdemo::memory::translate_addr;
        // let phys = unsafe {
        //     translate_addr(virt, phys_mem_offset)
        // };
        use x86_64::structures::paging::Translate;
        let phys = mapper.translate_addr(virt);
        println!("v: {:?} -> p: {:?}", virt, phys);
    }

    // 创建
    use x86_64::structures::paging::Page;
    let mut frame_allocator = memory::EmptyFrameAllocator;
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page,&mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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
use x86_64::{VirtAddr};

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}