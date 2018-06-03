use display::text_mode::LowDepthColour;
use display::text_mode::Printer;
use display::text_mode::SYSTEM_PRINTER;

pub struct BootStatus {
	updated: bool,
}

impl BootStatus {
	const STATUS_LENGTH: usize = 4;
	const FULL_STATUS_LENGTH: usize = Self::STATUS_LENGTH + 3;

	pub fn new(message: &'static str) -> BootStatus {
		let length = message.len() + Self::FULL_STATUS_LENGTH;
		print!("[ -- ] {}", message);
		for _ in 0..(length - 1) { print!("\x08"); }
		BootStatus {
			updated: false,
		}
	}

	fn reset_cursor(&mut self) -> &mut BootStatus {
		for _ in 0..Self::STATUS_LENGTH { print!("\x08"); }
		self.updated = true;
		self
	}

	pub fn with_message(&mut self) {
		SYSTEM_PRINTER.lock().print("\n       ");
	}

	pub fn set_failure(&mut self) -> &mut BootStatus {
		SYSTEM_PRINTER.lock().print_coloured("Fail", &LowDepthColour::BACKGROUND, &LowDepthColour::LightRed);
		self.reset_cursor()
	}

	pub fn set_warning(&mut self) -> &mut BootStatus {
		SYSTEM_PRINTER.lock().print_coloured("Warn", &LowDepthColour::BACKGROUND, &LowDepthColour::Yellow);
		self.reset_cursor()
	}

	pub fn set_success(&mut self) -> &mut Self {
		SYSTEM_PRINTER.lock().print_coloured(" Ok ", &LowDepthColour::BACKGROUND, &LowDepthColour::LightGreen);
		self.reset_cursor()
	}
}

impl Drop for BootStatus {
	fn drop(&mut self) {
		if !self.updated {
			self.set_success();
			println!();
		}
	}
}