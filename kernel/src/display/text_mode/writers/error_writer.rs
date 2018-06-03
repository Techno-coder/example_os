use core::fmt::Write;

pub static mut ERROR_WRITER: ErrorWriter = ErrorWriter {};

macro_rules! eprint {
    ($($arg:tt)*) => ({
        $crate::display::text_mode::writers::ErrorWriter::print(format_args!($($arg)*));
    });
}

macro_rules! eprintln {
	() => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

pub struct ErrorWriter {}

impl ErrorWriter {
	pub fn print(arguments: ::core::fmt::Arguments) {
		unsafe { ERROR_WRITER.write_fmt(arguments).unwrap() };
	}
}

impl ::core::fmt::Write for ErrorWriter {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		use display::text_mode::Printer;
		use display::text_mode::SYSTEM_PRINTER;
		SYSTEM_PRINTER.lock().print_error(s);
		Ok(())
	}
}
