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

    x86_64::instructions::interrupts::int3(); // breakpoint exception

    #[cfg(test)]
    test_main();
    
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pos::test_panic_handler(info)
}
