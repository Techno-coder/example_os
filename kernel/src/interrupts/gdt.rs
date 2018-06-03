use super::GdtDescriptor;
use x86_64::structures::gdt::SegmentSelector;

pub struct Gdt {
	table: [u64; 8],
	next_free_index: usize,
}

impl Gdt {
	pub const fn new() -> Gdt {
		Gdt {
			table: [0; 8],
			next_free_index: 1,
		}
	}

	pub fn load(&'static self) {
		use x86_64::instructions::tables::{DescriptorTablePointer, lgdt};
		use core::mem::size_of;

		let pointer = DescriptorTablePointer {
			base: self.table.as_ptr() as u64,
			limit: (self.table.len() * size_of::<u64>() - 1) as u16,
		};

		unsafe { lgdt(&pointer) };
	}

	pub fn add_kernel_entry(&mut self, entry: GdtDescriptor) -> SegmentSelector {
		let index = self.create_entry(entry);
		SegmentSelector::new(index, ::x86_64::PrivilegeLevel::Ring0)
	}

	pub fn add_user_entry(&mut self, entry: GdtDescriptor) -> SegmentSelector {
		let index = self.create_entry(entry);
		SegmentSelector::new(index, ::x86_64::PrivilegeLevel::Ring3)
	}

	fn create_entry(&mut self, entry: GdtDescriptor) -> u16 {
		match entry {
			GdtDescriptor::UserSegment(value) => self.push_value(value) as u16,
			GdtDescriptor::SystemSegment(value_low, value_high) => {
				let index = self.push_value(value_low);
				self.push_value(value_high);
				index as u16
			}
		}
	}

	fn push_value(&mut self, value: u64) -> usize {
		if self.next_free_index < self.table.len() {
			let index = self.next_free_index;
			self.table[index] = value;
			self.next_free_index += 1;
			index
		} else {
			panic!("GDT is full");
		}
	}
}

