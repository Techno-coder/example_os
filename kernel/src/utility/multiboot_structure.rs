use multiboot2::BootInformation;

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
