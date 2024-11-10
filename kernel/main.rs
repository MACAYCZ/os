#![no_main]
#![no_std]

mod vga;

use core::arch::global_asm;
use core::panic::PanicInfo;

#[repr(C)]
struct Multiboot {
	magic: u32,
	flags: u32,
	checksum: i32,
}

#[allow(dead_code)]
#[link_section = ".multiboot"]
static MULTIBOOT: Multiboot = Multiboot {
	magic: 0x1badb002,
	flags: 0x00,
	checksum: -(0x1badb002 + 0x00),
};

global_asm!(include_str!("entry.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
	println!("Hello, World!");
	loop {}
}
