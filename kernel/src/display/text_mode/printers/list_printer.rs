use display::text_mode::*;

pub struct ListPrinter<B: Buffer> {
	buffer: B,
}

impl<B: Buffer> ListPrinter<B> {
	pub fn new(buffer: B) -> ListPrinter<B> {
		ListPrinter {
			buffer,
		}
	}

	pub fn flush_all(&mut self) {
		self.buffer.flush_cells();
	}

	pub fn set_list<T: AsRef<str>>(&mut self, list: &[T]) {
		for row in 0..self.buffer.height() {
			let item = list.get(row).and_then(|item| Some(item.as_ref().as_bytes()));
			for column in 0..self.buffer.width() {
				let character = item.and_then(|item| item.get(column)).unwrap_or(&b' ');
				self.buffer[column][row].character = *character;
			}
		}
		self.buffer.flush_cells();
	}

	pub fn set_foreground_colour(&mut self, row: usize, colour: &LowDepthColour) {
		for column in 0..self.buffer.width() {
			self.buffer[column][row].foreground = *colour;
		}
		self.buffer.flush_row(row);
	}
}