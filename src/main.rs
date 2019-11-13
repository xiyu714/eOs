#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![allow(non_snake_case)]
#![feature(non_ascii_idents)]
#![feature(custom_test_frameworks)]     //启动自定义测试框架
#![test_runner(eOs::test_runner)]     //将自定义测试框架的runner函数设为`crate::test_runner`
/*
    本来测试框架会自动生成一个main函数作为入口，既主函数。然后执行runner函数
    但是这里使用了`#![no_main]`禁用了main函数，所以需要重新命名测试主函数，并在_start()函数中调用它
*/
#![reexport_test_harness_main = "test_main"]    //将自动生成的测试主函数命名为"test_main"

use eOs::println;

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为`_start`的函数，所以这个函数就是入口点
    // 默认命名为`_start`
    println!("Hello World{}", "!");

    eOs::init(); // new


    #[cfg(test)]
        test_main();


    println!("It did not crash!");

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
    eOs::test_panic_handler(info)
}
