use core::ops;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct TextCell {
	pub character: Character,
	pub foreground: LowDepthColour,
	pub background: LowDepthColour,
}

impl Default for TextCell {
	fn default() -> Self {
		TextCell {
			character: b' ',
			foreground: LowDepthColour::FOREGROUND,
			background: LowDepthColour::BACKGROUND,
		}
	}
}

pub trait Buffer: TextDisplay + ops::Index<usize, Output=[TextCell]> + ops::IndexMut<usize> {
	fn flush_cursor(&mut self);
	fn flush_cell(&mut self, position: &Position);

	fn flush_cells(&mut self) {
		for row in 0..self.height() {
			self.flush_row(row);
		}
	}

	fn flush_all(&mut self) {
		self.flush_cells();
		self.flush_cursor();
	}

	fn flush_row(&mut self, row: usize) {
		for column in 0..self.width() {
			self.flush_cell(&Position { column, row });
		}
	}
}