use core::fmt::Write;

pub static mut DEBUG_WRITER: DebugWriter = DebugWriter {};

macro_rules! debug_print {
    ($($arg:tt)*) => ({
        $crate::display::text_mode::writers::DebugWriter::print(format_args!($($arg)*));
    });
}

macro_rules! debug_println {
	() => (debug_print!("\n"));
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n"), $($arg)*));
}

pub struct DebugWriter {}

impl DebugWriter {
	pub fn print(arguments: ::core::fmt::Arguments) {
		unsafe { DEBUG_WRITER.write_fmt(arguments).unwrap() };
	}
}

impl ::core::fmt::Write for DebugWriter {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		const COMMUNICATION_PORT: u16 = 0x3f8;
		for byte in s.bytes() {
			unsafe {
				::x86_64::instructions::port::outb(COMMUNICATION_PORT, byte);
			}
		}
		Ok(())
	}
}
