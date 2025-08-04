#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
const COLORS: [u8; 7] = [0xc, 0xa, 0xe, 0x9, 0xd, 0xb, 0xf];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("i'm so panicked, right now...");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
