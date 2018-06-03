use super::LowDepthColour;

pub trait Printer {
	fn print(&mut self, string: &str) {
		self.print_coloured(string, &LowDepthColour::BACKGROUND, &LowDepthColour::FOREGROUND);
	}

	fn print_error(&mut self, string: &str) {
		self.print_coloured(string, &LowDepthColour::BACKGROUND, &LowDepthColour::LightRed);
	}

	fn print_coloured(&mut self, string: &str, background: &LowDepthColour, foreground: &LowDepthColour);
}
