use display::text_mode::*;
use super::ListPrinter;

pub struct SelectableList<B: Buffer> {
	list_printer: ListPrinter<B>,
	selected: usize,
	buffer_height: usize,
	list_size: usize,
}

impl<B: Buffer> SelectableList<B> {
	pub fn new(buffer: B) -> SelectableList<B> {
		let buffer_height = buffer.height();
		SelectableList {
			list_printer: ListPrinter::new(buffer),
			selected: 0,
			buffer_height,
			list_size: 0,
		}
	}

	pub fn flush_all(&mut self) {
		self.list_printer.flush_all();
	}

	pub fn set_list<T: AsRef<str>>(&mut self, list: &[T]) {
		self.list_printer.set_list(list);
		self.list_size = list.len().min(self.buffer_height);
	}

	pub fn set_selected(&mut self, index: usize) {
		self.list_printer.set_foreground_colour(self.selected, &LowDepthColour::FOREGROUND);
		self.list_printer.set_foreground_colour(index, &LowDepthColour::LightGreen);
		self.selected = index;
	}

	pub fn offset_selected(&mut self, offset: i32) {
		if self.list_size == 0 { return; }

		let selected = self.selected as i32;
		let mut new_selected = (selected + offset) % self.list_size as i32;
		if new_selected < 0 {
			new_selected = self.list_size as i32 + new_selected;
		}
		self.set_selected(new_selected as usize);
	}

	pub fn get_selected(&self) -> usize {
		self.selected
	}
}
