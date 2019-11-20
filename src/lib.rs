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
    hlt_loop();
}

/// Entry point for `cargo xtest`
#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
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

/**
    创建一个更加有效能的循环。
    hlt指令使CPU暂时休眠，直到外部中断的到来
    使用了这个方法以后CPU占用率大幅度降低。在Windows上，原来qemu运行CPU占用30%，改用hlt后，只要3%左右。
*/
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}