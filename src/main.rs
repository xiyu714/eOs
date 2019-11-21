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

use eOs::{println, hlt_loop};

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // 因为编译器会寻找一个名为`_start`的函数，所以这个函数就是入口点
    use eOs::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");

    eOs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
        test_main();


    println!("It did not crash!");

    hlt_loop();
}

/// 这个函数将在panic时被调用
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eOs::test_panic_handler(info)
}
