use display::text_mode::*;
use spin::Mutex;

pub struct DisplayFacade<'a, D: 'a> {
	display: &'a Mutex<D>,
}

impl<'a, D: TextDisplay> DisplayFacade<'a, D> {
	pub fn new(display: &'a Mutex<D>) -> DisplayFacade<'a, D> {
		DisplayFacade {
			display,
		}
	}
}

impl<'a, D: TextDisplay> TextDisplay for DisplayFacade<'a, D> {
	fn width(&self) -> Width {
		self.display.lock().width()
	}

	fn height(&self) -> Height {
		self.display.lock().height()
	}

	fn set_cell(&mut self, position: &Position, character: Character) {
		self.display.lock().set_cell(position, character)
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		self.display.lock().set_background(position, colour)
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		self.display.lock().set_foreground(position, colour)
	}

	fn set_cursor(&mut self, position: &Position) {
		self.display.lock().set_cursor(position)
	}
}
