use multiboot2::BootInformation;

// This structure is needed as multiboot2's ModuleIter cannot be cloned.
// See memory/huge_frame_allocators/huge_boot_bump_allocator for its
// main usage

#[derive(Debug, Clone)]
pub struct MultibootStructure {
	address: usize,
}

impl MultibootStructure {
	pub fn new(address: usize) -> MultibootStructure {
		MultibootStructure {
			address: address + ::KERNEL_BASE as usize,
		}
	}

	pub fn get(&self) -> BootInformation {
		unsafe { ::multiboot2::load(self.address) }
	}
}
