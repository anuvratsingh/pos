#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "World!");

    pos::init();

    // x86_64::instructions::interrupts::int3();

    // let ptr = 0x2058a3 as *mut u32;
    // unsafe {
    //     let x = *ptr;
    // };
    // println!("Read worked");
    // unsafe { *ptr = 42 };
    // println!("Write Worked");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table);

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    pos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pos::test_panic_handler(info)
}
