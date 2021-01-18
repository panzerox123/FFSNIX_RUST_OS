#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga;

//static TEST: &[u8] = b"FFNIX_RUST";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::writeString(b"Hello, World", 0);
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
