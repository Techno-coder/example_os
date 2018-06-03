use x86_64::instructions::port::outb;

pub const PIC_ONE_VECTOR_BASE: u8 = 32;
pub const PIC_TWO_VECTOR_BASE: u8 = 40;

const PIC_ONE_COMMAND_PORT: u16 = 0x20;
const PIC_TWO_COMMAND_PORT: u16 = 0xa0;

const PIC_ONE_DATA_PORT: u16 = 0x21;
const PIC_TWO_DATA_PORT: u16 = 0xa1;

pub fn initialize() {
	unsafe {
		remap_pic();
		mask_pic();
	}
}

unsafe fn remap_pic() {
	use x86_64::instructions::port::outb;
	const PIC_RESTART_COMMAND: u8 = 0x11;

	outb(PIC_ONE_COMMAND_PORT, PIC_RESTART_COMMAND);
	outb(PIC_TWO_COMMAND_PORT, PIC_RESTART_COMMAND);

	outb(PIC_ONE_DATA_PORT, PIC_ONE_VECTOR_BASE);
	outb(PIC_TWO_DATA_PORT, PIC_TWO_VECTOR_BASE);

	outb(PIC_ONE_DATA_PORT, 0x04);
	outb(PIC_TWO_DATA_PORT, 0x02);

	outb(PIC_ONE_DATA_PORT, 0x01);
	outb(PIC_ONE_DATA_PORT, 0x01);
}

unsafe fn mask_pic() {
	outb(PIC_ONE_DATA_PORT, 0b1111_1110);
	outb(PIC_TWO_DATA_PORT, 0b1111_1111);
}

pub fn send_interrupt_end(both_chips: bool) {
	unsafe {
		if both_chips {
			outb(PIC_TWO_COMMAND_PORT, 0x20);
		}
		outb(PIC_ONE_COMMAND_PORT, 0x20);
	}
}