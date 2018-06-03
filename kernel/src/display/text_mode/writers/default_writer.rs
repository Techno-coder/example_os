use core::fmt::Write;

pub static mut DEFAULT_WRITER: DefaultWriter = DefaultWriter {};

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::display::text_mode::writers::DefaultWriter::print(format_args!($($arg)*));
    });
}

macro_rules! println {
	() => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub struct DefaultWriter {}

impl DefaultWriter {
	pub fn print(arguments: ::core::fmt::Arguments) {
		unsafe { DEFAULT_WRITER.write_fmt(arguments).unwrap() };
	}
}

impl ::core::fmt::Write for DefaultWriter {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		use display::text_mode::Printer;
		use display::text_mode::SYSTEM_PRINTER;
		SYSTEM_PRINTER.lock().print(s);
		Ok(())
	}
}
