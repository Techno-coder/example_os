use memory::Frame;
use memory::FrameLikeAllocator;
use super::EntryFlags;
use super::PageEntry;
use super::TableLevel;
use super::VirtualAddress;

const ENTRY_COUNT: usize = 512;

pub struct PageTable<L: TableLevel> {
	entries: [PageEntry; ENTRY_COUNT],
	_level: ::core::marker::PhantomData<L>,
}

impl<L> PageTable<L> where L: TableLevel {
	pub fn clear(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.set_unused();
		}
	}
}

impl<L> PageTable<L> where L: super::HierarchicalLevel {
	fn next_table_address(&self, index: usize) -> Option<VirtualAddress> {
		let entry_flags = self[index].flags();
		if entry_flags.contains(EntryFlags::PRESENT) && !entry_flags.contains(EntryFlags::HUGE_PAGE) {
			let table_address = self as *const _ as usize;
			return Some(VirtualAddress::new((table_address << 9) | (index << 12)));
		}
		None
	}

	pub fn next_table(&self, index: usize) -> Option<&PageTable<L::NextLevel>> {
		self.next_table_address(index).map(|address| unsafe { &*(address.raw() as *const _) })
	}

	pub fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable<L::NextLevel>> {
		self.next_table_address(index).map(|address| unsafe { &mut *(address.raw() as *mut _) })
	}

	pub fn create_if_nonexistent(&mut self, index: usize, allocator: &mut FrameLikeAllocator<Frame>) -> &mut PageTable<L::NextLevel> {
		if self.next_table_mut(index).is_none() {
			let table_frame = allocator.allocate().expect("Out of memory: PageTable entry");

			// Note well: The flags must be USER_ACCESSIBLE because all the
			// page directories have to be USER_ACCESSIBLE for a user mode
			// thread to access a USER_ACCESSIBLE page
			let flags = EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE;
			self.entries[index].set(table_frame, flags);
			self.next_table_mut(index).unwrap().clear()
		}

		self.next_table_mut(index).unwrap()
	}
}

impl<L> ::core::ops::Index<usize> for PageTable<L> where L: TableLevel {
	type Output = PageEntry;

	fn index(&self, index: usize) -> &PageEntry {
		&self.entries[index]
	}
}

impl<L> ::core::ops::IndexMut<usize> for PageTable<L> where L: TableLevel {
	fn index_mut(&mut self, index: usize) -> &mut PageEntry {
		&mut self.entries[index]
	}
}