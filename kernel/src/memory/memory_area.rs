use super::PhysicalAddress;

#[derive(Debug, Clone)]
pub struct MemoryArea {
	start: PhysicalAddress,
	size: usize,
}

impl MemoryArea {
	pub fn new(start: PhysicalAddress, size: usize) -> MemoryArea {
		MemoryArea {
			start,
			size,
		}
	}

	pub fn start_address(&self) -> PhysicalAddress {
		self.start.clone()
	}

	pub fn end_address(&self) -> PhysicalAddress {
		PhysicalAddress::new(self.start.raw() + self.size() as u64)
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn overlap(&self, other: &MemoryArea) -> Option<MemoryArea> {
		let start = self.start_address().raw().max(other.start_address().raw());
		let end = self.end_address().raw().min(other.end_address().raw());
		let size = end.checked_sub(start)? as usize;
		Some(MemoryArea::new(PhysicalAddress::new(start), size))
	}
}

impl<'a> From<&'a ::multiboot2::MemoryArea> for MemoryArea {
	fn from(other: &::multiboot2::MemoryArea) -> Self {
		let start = PhysicalAddress::new(other.start_address() as u64);
		MemoryArea::new(start, other.size())
	}
}

impl<'a> From<&'a ::multiboot2::ModuleTag> for MemoryArea {
	fn from(other: &::multiboot2::ModuleTag) -> Self {
		let start = PhysicalAddress::new(other.start_address() as u64);
		MemoryArea::new(start, (other.end_address() - other.start_address()) as usize)
	}
}
