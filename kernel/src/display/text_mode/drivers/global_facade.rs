use display::text_mode::*;
use display::text_mode::functions::SYSTEM_DISPLAY;

// This structure allows multiple printers to use
// the same display driver
pub struct GlobalFacade;

impl TextDisplay for GlobalFacade {
	fn width(&self) -> usize {
		SYSTEM_DISPLAY.lock().width()
	}

	fn height(&self) -> usize {
		SYSTEM_DISPLAY.lock().height()
	}

	fn set_cell(&mut self, position: &Position, character: u8) {
		SYSTEM_DISPLAY.lock().set_cell(position, character);
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		SYSTEM_DISPLAY.lock().set_background(position, colour);
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		SYSTEM_DISPLAY.lock().set_foreground(position, colour);
	}

	fn set_cursor(&mut self, position: &Position) {
		SYSTEM_DISPLAY.lock().set_cursor(position);
	}
}
