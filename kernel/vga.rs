use core::fmt;
use core::ptr;

const ROWS: usize = 25;
const COLS: usize = 80;

#[repr(transparent)]
pub struct Console {
	cursor: usize,
}

impl Console {
	fn putc(&mut self, c: u8) {
		let buffer = 0xB8000 as *mut u16;
		match c {
			b'\n' => {
				self.cursor = (self.cursor / COLS + 1) * COLS;
			}
			c => unsafe {
				ptr::write_volatile(buffer.offset(self.cursor as isize), (c as u16) | 0x0700);
				self.cursor += 1;
			}
		}
		if self.cursor >= ROWS * COLS {
			self.cursor = (ROWS - 1) * COLS;
			unsafe {
				ptr::copy(buffer, buffer.offset(COLS as isize), (ROWS - 1) * COLS);
			}
		}
	}

	#[allow(dead_code)]
	pub fn print(&mut self, args: fmt::Arguments) {
		use core::fmt::Write;
		self.write_fmt(args).unwrap();
	}
}

impl fmt::Write for Console {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for c in s.bytes() {
			self.putc(c);
		}
		Ok(())
	}
}

#[allow(dead_code)]
pub static mut CONSOLE: Console = Console {
	cursor: 0,
};

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (unsafe {
		#[allow(static_mut_refs)]
		$crate::vga::CONSOLE.print(format_args!($($arg)*)) 
	});
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
