#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
	pub const fn new(raw: usize) -> VirtualAddress {
		VirtualAddress(raw)
	}

	/// Creates a new VirtualAddress in the higher half
	pub const fn new_adjusted(raw: usize) -> VirtualAddress {
		VirtualAddress::new(raw + ::KERNEL_BASE as usize)
	}

	pub const fn raw(&self) -> usize {
		self.0
	}

	pub const fn offset(&self, offset: usize) -> VirtualAddress {
		VirtualAddress::new(self.raw() + offset)
	}
}

impl ::core::fmt::Debug for VirtualAddress {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
		write!(f, "{:#x}", self.raw())
	}
}

impl From<::x86_64::VirtualAddress> for VirtualAddress {
	fn from(address: ::x86_64::VirtualAddress) -> Self {
		Self::new(address.0)
	}
}
