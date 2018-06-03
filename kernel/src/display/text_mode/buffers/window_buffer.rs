use alloc::Vec;
use display::text_mode::*;

pub struct WindowBuffer<D: TextDisplay> {
	display: D,

	position: Position,
	width: Width,
	height: Height,

	buffer: Vec<TextCell>,
	cursor: Position,
}

impl<D: TextDisplay> WindowBuffer<D> {
	pub fn new(display: D, position: Position, width: Width, height: Height) -> WindowBuffer<D> {
		WindowBuffer {
			display,
			position,
			width,
			height,
			buffer: vec![TextCell::default(); width * height],
			cursor: Position::default(),
		}
	}
}

impl<D: TextDisplay> Buffer for WindowBuffer<D> {
	fn flush_cursor(&mut self) {
		self.display.set_cursor(&(&self.cursor + &self.position))
	}

	fn flush_cell(&mut self, position: &Position) {
		let text_cell = &self.buffer[self.height * position.column + position.row];
		let position = &(position + &self.position);
		self.display.set_cell(position, text_cell.character);
		self.display.set_background(position, &text_cell.background);
		self.display.set_foreground(position, &text_cell.foreground);
	}
}

impl<D: TextDisplay> TextDisplay for WindowBuffer<D> {
	fn width(&self) -> Width {
		self.width
	}

	fn height(&self) -> Height {
		self.height
	}

	fn set_cell(&mut self, position: &Position, character: Character) {
		self[position.column][position.row].character = character;
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		self[position.column][position.row].background = *colour;
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		self[position.column][position.row].foreground = *colour;
	}

	fn set_cursor(&mut self, position: &Position) {
		self.cursor = position.clone();
	}
}

impl<D: TextDisplay> ::core::ops::Index<usize> for WindowBuffer<D> {
	type Output = [TextCell];

	fn index(&self, index: usize) -> &[TextCell] {
		let column = self.height * index;
		&self.buffer[column..(column + self.height)]
	}
}

impl<D: TextDisplay> ::core::ops::IndexMut<usize> for WindowBuffer<D> {
	fn index_mut(&mut self, index: usize) -> &mut [TextCell] {
		let column = self.height * index;
		&mut self.buffer[column..(column + self.height)]
	}
}
