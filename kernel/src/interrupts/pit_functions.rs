const BASE_FREQUENCY: u32 = 1_193_180;
const COMMAND_PORT: u16 = 0x43;
const DATA_PORT: u16 = 0x40;

pub fn initialize() {
	const DIVIDER_FREQUENCY: u32 = 100;
	unsafe {
		set_frequency(DIVIDER_FREQUENCY);
	}
}

unsafe fn set_frequency(hertz: u32) {
	use x86_64::instructions::port::outb;

	let division = BASE_FREQUENCY / hertz;
	let mode = 0b0011_0110;

	let low = division & 0x0000_00ff;
	let high = (division & 0x0000_ff00) >> 8;

	outb(COMMAND_PORT, mode);
	outb(DATA_PORT, low as u8);
	outb(DATA_PORT, high as u8);
}
