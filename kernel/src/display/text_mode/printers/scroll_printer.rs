use display::text_mode::*;

pub struct ScrollPrinter<B: Buffer> {
	buffer: B,
	column: usize,
	row: usize,
}

impl<B: Buffer> ScrollPrinter<B> {
	pub fn new(buffer: B) -> ScrollPrinter<B> {
		let final_row = buffer.height() - 1;
		ScrollPrinter {
			buffer,
			column: 0,
			row: final_row,
		}
	}

	pub fn flush_all(&mut self) {
		self.buffer.flush_all();
	}

	fn update_cursor(&mut self) {
		self.buffer.set_cursor(&Position {
			column: self.column,
			row: self.row,
		});
	}

	fn new_line(&mut self) {
		for row in 1..self.buffer.height() {
			for column in 0..self.buffer.width() {
				self.buffer[column][row - 1] = self.buffer[column][row];
			}
		}
		let final_row = self.final_row();
		self.clear_row(final_row);
		self.column = 0;
	}

	fn backspace(&mut self) {
		if self.column > 0 {
			self.column -= 1;
		}
	}

	fn clear_row(&mut self, row: usize) {
		for column in 0..self.buffer.width() {
			self.buffer[column][row] = TextCell::default();
		}
	}

	fn final_row(&self) -> usize {
		self.buffer.height() - 1
	}
}

impl<B: Buffer> Printer for ScrollPrinter<B> {
	fn print_coloured(&mut self, string: &str, background: &LowDepthColour, foreground: &LowDepthColour) {
		let mut new_line_printed = false;
		for character in string.bytes() {
			match character {
				b'\n' => {
					self.new_line();
					new_line_printed = true;
				}
				b'\x08' => {
					self.backspace();
				}
				_ => {
					if self.column == self.buffer.width() {
						self.new_line();
						new_line_printed = true;
					}

					let cell = &mut self.buffer[self.column][self.row];
					cell.character = character;
					cell.background = *background;
					cell.foreground = *foreground;

					self.column += 1;
				}
			}
		}

		self.update_cursor();
		if new_line_printed {
			self.buffer.flush_all();
		} else {
			self.buffer.flush_row(self.row);
			self.buffer.flush_cursor();
		}
	}
}
