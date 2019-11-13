#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![allow(non_snake_case)]
#![feature(non_ascii_idents)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();      // new
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

/*
    在这里使用qemu提供的 isa-debug-exit设备。
    当一个 value被写入iobase指定的端口时，它将导致QEMU以退出状态（exit status）(value << 1) | 1退出。
    也就是说，当我们向端口写入0时，QEMU将以退出状态(0 << 1) | 1 = 1退出；而当我们向端口写入1时，它将以退出状态(1 << 1) | 1 = 3退出。

    这里还有一个问题，就是因为qemu会将退出状态改变，这使得不能使实际退出码为0
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, // 这里不能使用0，因为0会被转换为1，而1是qemu运行失败时的默认退出代码
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
