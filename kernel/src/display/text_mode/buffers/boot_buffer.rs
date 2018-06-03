use display::text_mode::*;

const WIDTH: Width = 80;
const HEIGHT: Height = 25;

pub struct BootBuffer<D: TextDisplay> {
	display: D,
	buffer: [[TextCell; HEIGHT]; WIDTH],
	cursor: Position,
}

impl<D: TextDisplay> BootBuffer<D> {
	pub fn new(display: D) -> BootBuffer<D> {
		BootBuffer {
			display,
			buffer: [[TextCell::default(); 25]; 80],
			cursor: Position::default(),
		}
	}
}

impl<D: TextDisplay> Buffer for BootBuffer<D> {
	fn flush_cursor(&mut self) {
		self.display.set_cursor(&self.cursor);
	}

	fn flush_cell(&mut self, position: &Position) {
		let text_cell = &self.buffer[position.column][position.row];
		self.display.set_cell(position, text_cell.character);
		self.display.set_background(position, &text_cell.background);
		self.display.set_foreground(position, &text_cell.foreground);
	}
}

impl<D: TextDisplay> TextDisplay for BootBuffer<D> {
	fn width(&self) -> Width {
		WIDTH
	}

	fn height(&self) -> Height {
		HEIGHT
	}

	fn set_cell(&mut self, position: &Position, character: Character) {
		self.buffer[position.column][position.row].character = character;
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		self.buffer[position.column][position.row].background = *colour;
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		self.buffer[position.column][position.row].foreground = *colour;
	}

	fn set_cursor(&mut self, position: &Position) {
		self.cursor = position.clone();
	}
}

impl<D: TextDisplay> ::core::ops::Index<usize> for BootBuffer<D> {
	type Output = [TextCell];

	fn index(&self, index: usize) -> &[TextCell] {
		&self.buffer[index]
	}
}

impl<D: TextDisplay> ::core::ops::IndexMut<usize> for BootBuffer<D> {
	fn index_mut(&mut self, index: usize) -> &mut [TextCell] {
		&mut self.buffer[index]
	}
}
