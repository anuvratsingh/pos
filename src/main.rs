#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write; // trait for WRITER
    vga_buffer::WRITER.lock().write_str("Hello").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some num: {} {}", 42, 1.337).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
