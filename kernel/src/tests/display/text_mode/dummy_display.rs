use display::text_mode::*;

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

type Buffer = [[TextCell; WIDTH]; HEIGHT];

pub struct DummyDisplay {
	pub buffer: Buffer,
	pub cursor: Position,
}

impl DummyDisplay {
	pub fn new() -> DummyDisplay {
		DummyDisplay {
			buffer: [[TextCell::default(); WIDTH]; HEIGHT],
			cursor: Position { column: 0, row: 0 },
		}
	}

	pub fn cell(&self, position: &Position) -> &TextCell {
		&self.buffer[position.row][position.column]
	}
}

impl TextDisplay for DummyDisplay {
	fn width(&self) -> Width {
		WIDTH
	}

	fn height(&self) -> Height {
		HEIGHT
	}

	fn set_cell(&mut self, position: &Position, character: Character) {
		self.buffer[position.row][position.column].character = character;
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		self.buffer[position.row][position.column].background = *colour;
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		self.buffer[position.row][position.column].foreground = *colour;
	}

	fn set_cursor(&mut self, position: &Position) {
		self.cursor = position.clone();
	}
}
