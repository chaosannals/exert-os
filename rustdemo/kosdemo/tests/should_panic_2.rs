#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kosdemo::{QemuExitCode, exit_qemu, serial_println, serial_print};

// 当 panic 触发 测试成功。
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}