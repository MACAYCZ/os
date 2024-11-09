#![no_main]
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
	unsafe {
		*(0xB8000 as *mut u8) = b'X';
	}
	loop {}
}
