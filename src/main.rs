#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![allow(non_snake_case)]
#![feature(non_ascii_idents)]
#![feature(custom_test_frameworks)]     //启动自定义测试框架
#![test_runner(crate::test_runner)]     //将自定义测试框架的runner函数设为`crate::test_runner`
/*
    本来测试框架会自动生成一个main函数作为入口，既主函数。然后执行runner函数
    但是这里使用了`#![no_main]`禁用了main函数，所以需要重新命名测试主函数，并在_start()函数中调用它
*/
#![reexport_test_harness_main = "test_main"]    //将自动生成的测试主函数命名为"test_main"

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为`_start`的函数，所以这个函数就是入口点
    // 默认命名为`_start`
    println!("Hello World{}", "!");

    #[cfg(test)]
        test_main();

    loop {}
}

/// 这个函数将在panic时被调用
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}