use display::text_mode::*;

type Buffer = [[VgaCharacter; VgaTextDriver::WIDTH]; VgaTextDriver::HEIGHT];

pub struct VgaTextDriver {
	buffer: &'static mut Buffer,
}

impl VgaTextDriver {
	const WIDTH: Width = 80;
	const HEIGHT: Height = 25;

	const COMMAND_PORT: u16 = 0x3d4;
	const DATA_PORT: u16 = 0x3d5;

	/// Creates an interface over the VGA frame buffer
	///
	/// # Safety
	///
	/// Only one VgaTextDriver instance should exist
	/// The VGA buffer should be paged at `0xb8000 + KERNEL_BASE`
	///
	pub unsafe fn new() -> VgaTextDriver {
		VgaTextDriver::enable_cursor();
		VgaTextDriver {
			// The buffer page is located in the higher half
			buffer: &mut *((0xb8000 + ::KERNEL_BASE) as *mut Buffer),
		}
	}

	fn match_colour_code(colour: &LowDepthColour) -> u8 {
		match *colour {
			LowDepthColour::Black => 0,
			LowDepthColour::Blue => 1,
			LowDepthColour::Green => 2,
			LowDepthColour::Cyan => 3,
			LowDepthColour::Red => 4,
			LowDepthColour::Magenta => 5,
			LowDepthColour::Brown => 6,
			LowDepthColour::LightGray => 7,
			LowDepthColour::DarkGray => 8,
			LowDepthColour::LightBlue => 9,
			LowDepthColour::LightGreen => 10,
			LowDepthColour::LightCyan => 11,
			LowDepthColour::LightRed => 12,
			LowDepthColour::Pink => 13,
			LowDepthColour::Yellow => 14,
			LowDepthColour::White => 15,
		}
	}

	fn enable_cursor() {
		use x86_64::instructions::port::outb;
		use x86_64::instructions::port::inb;
		unsafe {
			outb(VgaTextDriver::COMMAND_PORT, 0x0A);
			outb(VgaTextDriver::DATA_PORT, inb(VgaTextDriver::DATA_PORT) & 0xC0);
			outb(VgaTextDriver::COMMAND_PORT, 0x0B);
			outb(VgaTextDriver::DATA_PORT, (inb(0x3E0) & 0xE0) | (VgaTextDriver::HEIGHT as u8 - 1));
		}
	}
}

impl TextDisplay for VgaTextDriver {
	fn width(&self) -> Width {
		VgaTextDriver::WIDTH
	}

	fn height(&self) -> Height {
		VgaTextDriver::HEIGHT
	}

	fn set_cell(&mut self, position: &Position, character: Character) {
		self.buffer[position.row][position.column].character = character;
	}

	fn set_background(&mut self, position: &Position, colour: &LowDepthColour) {
		let buffer_colour = &mut self.buffer[position.row][position.column].colour;
		*buffer_colour &= 0b1111_0000;
		*buffer_colour |= VgaTextDriver::match_colour_code(colour) << 4;
	}

	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour) {
		let buffer_colour = &mut self.buffer[position.row][position.column].colour;
		*buffer_colour &= 0b0000_1111;
		*buffer_colour |= VgaTextDriver::match_colour_code(colour);
	}

	fn set_cursor(&mut self, position: &Position) {
		use x86_64::instructions::port::outb;
		use x86_64::instructions::port::outw;
		let position = (position.row * VgaTextDriver::WIDTH) + position.column;
		unsafe {
			outb(VgaTextDriver::COMMAND_PORT, 0x0f);
			outw(VgaTextDriver::DATA_PORT, position as u16 & 0xff);
			outb(VgaTextDriver::COMMAND_PORT, 0x0e);
			outw(VgaTextDriver::DATA_PORT, (position as u16 >> 8) & 0xff);
		}
	}
}

#[repr(C)]
struct VgaCharacter {
	character: u8,
	colour: u8,
}
