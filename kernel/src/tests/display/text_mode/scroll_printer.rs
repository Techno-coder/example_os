#[test]
fn test_scroll_printer() {
	use display::text_mode::*;
	use display::text_mode::printers::scroll_printer::ScrollPrinter;
	use display::text_mode::buffers::BootBuffer;

	let display = ::spin::Mutex::new(super::DummyDisplay::new());
	let facade = super::DisplayFacade::new(&display);
	let buffer = BootBuffer::new(facade);
	let mut printer = ScrollPrinter::new(buffer);

	{
		let string = "This is a test";
		printer.print(string);
		printer.print("\n");
		let display = display.lock();
		for (column, character) in string.bytes().enumerate() {
			let position = Position {
				column,
				row: 23,
			};
			assert_eq!(display.cell(&position).character, character);
		}
	}

	{
		let string = "The second line?";
		printer.print(string);
		let display = display.lock();
		for (column, character) in string.bytes().enumerate() {
			let position = Position {
				column,
				row: 24,
			};
			assert_eq!(display.cell(&position).character, character);
		}
	}
}