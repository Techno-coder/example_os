#[test]
fn test_fixed_buffer() {
	use display::text_mode::*;
	use display::text_mode::buffers::BootBuffer;

	let display = ::spin::Mutex::new(super::DummyDisplay::new());
	let facade = super::DisplayFacade::new(&display);
	let mut fixed_buffer = BootBuffer::new(facade);

	{
		let position = Position { column: 79, row: 24 };
		fixed_buffer.set_cell(&position, b'A');
		fixed_buffer.flush_cell(&position);
		assert_eq!(display.lock().cell(&position).character, b'A');

		fixed_buffer.set_cell(&position, b'B');
		assert_ne!(display.lock().cell(&position).character, b'B');
	}

	{
		let position = Position { column: 0, row: 0 };
		fixed_buffer.set_foreground(&position, &LowDepthColour::Green);
		fixed_buffer.flush_cell(&position);
		assert_eq!(display.lock().cell(&position).foreground, LowDepthColour::Green);
	}

	{
		let position = Position { column: 50, row: 10 };
		fixed_buffer.set_background(&position, &LowDepthColour::Red);
		fixed_buffer.flush_cell(&position);
		assert_eq!(display.lock().cell(&position).background, LowDepthColour::Red);
	}

	{
		let position = Position { column: 25, row: 14 };
		fixed_buffer.set_cursor(&position);
		fixed_buffer.flush_cursor();
		assert_eq!(display.lock().cursor, position);
	}
}