#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga;

//static TEST: &[u8] = b"FFNIX_RUST";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let a = vga::textBlock {
        text: 'm' as u8,
        fgColor: vga::Color::Blue,
        bgColor: vga::Color::Black,
    };
    vga::writeByte(a);
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
