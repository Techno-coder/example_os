use alloc::String;
use display::text_mode::*;
use display::text_mode::buffers::BootBuffer;
use display::text_mode::buffers::WindowBuffer;
use display::text_mode::drivers::GlobalFacade;
use display::text_mode::printers::ScrollPrinter;
use display::text_mode::printers::SelectableList;

pub struct ShellDisplay {
	display: BootBuffer<GlobalFacade>,
	main_panel: ScrollPrinter<WindowBuffer<GlobalFacade>>,
	list_panel: SelectableList<WindowBuffer<GlobalFacade>>,
}

impl ShellDisplay {
	const SIDEBAR_WIDTH: usize = 20;

	pub fn new() -> ShellDisplay {
		let display = BootBuffer::new(GlobalFacade);
		let main_panel = {
			let position = Position { column: 0, row: 0 };
			let width = display.width() - Self::SIDEBAR_WIDTH;
			let height = display.height();
			let buffer = WindowBuffer::new(GlobalFacade, position, width, height);
			ScrollPrinter::new(buffer)
		};
		let mut list_panel = {
			let position = Position { column: display.width() - (Self::SIDEBAR_WIDTH - 1), row: 0 };
			let width = Self::SIDEBAR_WIDTH - 1;
			let height = display.height();
			let buffer = WindowBuffer::new(GlobalFacade, position, width, height);
			SelectableList::new(buffer)
		};
		list_panel.set_selected(0);
		ShellDisplay {
			display,
			main_panel,
			list_panel,
		}
	}

	pub fn main_width(&self) -> usize {
		self.display.width() - Self::SIDEBAR_WIDTH
	}

	pub fn redraw(&mut self) {
		self.draw_interface();
		self.list_panel.flush_all();
		self.main_panel.flush_all();
	}

	fn draw_interface(&mut self) {
		let separator_x = self.display.width() - Self::SIDEBAR_WIDTH;
		for y in 0..self.display.height() {
			self.display[separator_x][y].character = b'|';
		}
		self.display.flush_all();
	}

	pub fn print(&mut self, string: &str) {
		self.main_panel.print(string);
	}

	pub fn print_error(&mut self, string: &str) {
		self.main_panel.print_error(string);
	}

	pub fn push_character(&mut self, character: char) {
		use alloc::string::ToString;
		self.main_panel.print(&character.to_string());
		self.list_panel.set_selected(0);
	}

	pub fn pop_character(&mut self) {
		self.main_panel.print("\x08 \x08");
		self.list_panel.set_selected(0);
	}

	pub fn set_options(&mut self, options: &[String]) {
		self.list_panel.set_list(options);
	}

	pub fn offset_selected(&mut self, offset: i32) {
		self.list_panel.offset_selected(offset);
	}

	pub fn get_selected(&self) -> usize {
		self.list_panel.get_selected()
	}
}