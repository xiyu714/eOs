#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![allow(non_snake_case)]
#![feature(non_ascii_idents)]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为`_start`的函数，所以这个函数就是入口点
    // 默认命名为`_start`
    println!("Hello World{}", "!");
    loop {}
}

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}