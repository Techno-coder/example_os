#[derive(Debug, Clone)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
	pub fn new(raw: u64) -> PhysicalAddress {
		assert!(raw < ::KERNEL_BASE, "Physical address {:#x} is greater than ::KERNEL_BASE", raw);
		PhysicalAddress::new_unchecked(raw)
	}

	/// Creates a PhysicalAddress from a higher half address
	pub fn new_force_adjust(raw: u64) -> PhysicalAddress {
		assert!(raw >= ::KERNEL_BASE, "Physical address {:#x} is not a higher half address", raw);
		PhysicalAddress::new_unchecked(raw - ::KERNEL_BASE as u64)
	}

	/// Creates a PhysicalAddress and adjusts it if it is a higher half address
	pub fn new_adjusted(raw: u64) -> PhysicalAddress {
		if raw >= ::KERNEL_BASE {
			return PhysicalAddress::new_force_adjust(raw);
		}
		PhysicalAddress::new(raw)
	}

	pub fn new_unchecked(raw: u64) -> PhysicalAddress {
		PhysicalAddress(raw)
	}

	pub fn raw(&self) -> u64 {
		self.0
	}

	pub fn align_up(&self, multiple: u64) -> PhysicalAddress {
		PhysicalAddress::new(::utility::math::align_up_u64(self.raw(), multiple))
	}

	pub fn align_down(&self, multiple: u64) -> PhysicalAddress {
		PhysicalAddress::new(::utility::math::align_down_u64(self.raw(), multiple))
	}
}

impl PartialEq for PhysicalAddress {
	fn eq(&self, other: &Self) -> bool {
		self.raw().eq(&other.raw())
	}
}

impl PartialOrd for PhysicalAddress {
	fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
		Some(self.raw().cmp(&other.raw()))
	}
}
