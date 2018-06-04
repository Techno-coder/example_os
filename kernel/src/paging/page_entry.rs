use memory::FrameLike;
use memory::PhysicalAddress;

pub struct PageEntry(u64);

impl PageEntry {
	const ADDRESS_MASK: u64 = 0x000f_ffff_ffff_f000;

	pub fn raw(&self) -> u64 {
		self.0
	}

	fn set_raw(&mut self, raw: u64) {
		self.0 = raw;
	}

	pub fn is_unused(&self) -> bool {
		self.raw() == 0
	}

	pub fn set_unused(&mut self) {
		self.set_raw(0);
	}

	pub fn flags(&self) -> EntryFlags {
		EntryFlags::from_bits_truncate(self.raw())
	}

	fn address(&self) -> PhysicalAddress {
		PhysicalAddress::new(self.raw() & Self::ADDRESS_MASK)
	}

	pub fn frame<F: FrameLike>(&self) -> Option<F> {
		if self.flags().contains(EntryFlags::PRESENT) {
			return Some(F::from_address(self.address()));
		}
		None
	}

	pub fn set<F: FrameLike>(&mut self, frame: F, flags: EntryFlags) {
		assert_eq!(frame.start_address().raw() & !Self::ADDRESS_MASK, 0);
		self.set_raw(frame.start_address().raw() as u64 | flags.bits());
	}
}

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const NO_EXECUTE =      1 << 63;
    }
}

impl EntryFlags {
	pub fn kernel_elf_section(section: &::multiboot2::ElfSection) -> EntryFlags {
		use multiboot2::ElfSectionFlags;
		let mut flags = EntryFlags::empty();

		if section.flags().contains(ElfSectionFlags::ALLOCATED) {
			flags |= Self::PRESENT;
		}

		if section.flags().contains(ElfSectionFlags::WRITABLE) {
			flags |= Self::WRITABLE;
		}

		if !section.flags().contains(ElfSectionFlags::EXECUTABLE) {
			flags |= Self::NO_EXECUTE;
		}

		flags
	}
}
