use x86_64::structures::tss::TaskStateSegment;

pub enum GdtDescriptor {
	UserSegment(u64),
	SystemSegment(u64, u64),
}

impl GdtDescriptor {
	pub fn kernel_code_segment() -> GdtDescriptor {
		let flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT |
			DescriptorFlags::EXECUTABLE | DescriptorFlags::LONG_MODE;
		GdtDescriptor::UserSegment(flags.bits())
	}

	pub fn kernel_data_segment() -> GdtDescriptor {
		let flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT |
			DescriptorFlags::LONG_MODE;
		GdtDescriptor::UserSegment(flags.bits())
	}

	pub fn user_code_segment() -> GdtDescriptor {
		let flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT |
			DescriptorFlags::EXECUTABLE | DescriptorFlags::LONG_MODE | DescriptorFlags::RING_USER;
		GdtDescriptor::UserSegment(flags.bits())
	}

	pub fn user_data_segment() -> GdtDescriptor {
		let flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT |
			DescriptorFlags::LONG_MODE | DescriptorFlags::WRITABLE | DescriptorFlags::RING_USER;
		GdtDescriptor::UserSegment(flags.bits())
	}

	pub fn tss_segment(tss: &TaskStateSegment) -> GdtDescriptor {
		use core::mem::size_of;
		use bit_field::BitField;
		let tss_pointer = tss as *const _ as u64;

		let mut low = DescriptorFlags::PRESENT.bits();
		low.set_bits(16..40, tss_pointer.get_bits(0..24));
		low.set_bits(56..64, tss_pointer.get_bits(24..32));
		low.set_bits(0..16, (size_of::<TaskStateSegment>() - 1) as u64);
		low.set_bits(40..44, 0b1001);

		let mut high = 0;
		high.set_bits(0..32, tss_pointer.get_bits(32..64));
		GdtDescriptor::SystemSegment(low, high)
	}
}

bitflags! {
    struct DescriptorFlags: u64 {
        const WRITABLE      = 1 << 41;
        const CONFORMING    = 1 << 42;
        const EXECUTABLE    = 1 << 43;
        const USER_SEGMENT  = 1 << 44;
        const RING_USER     = 3 << 45;
        const PRESENT       = 1 << 47;
        const LONG_MODE     = 1 << 53;
    }
}
