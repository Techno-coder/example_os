use memory::Frame;
use memory::frame_allocators::TinyAllocator;
use super::Page;
use super::PageMapper;
use super::PageTable;
use super::table_level::Level1;
use super::VirtualAddress;

pub struct TemporaryPage {
	page: Page,
	allocator: TinyAllocator,
}

impl TemporaryPage {
	pub fn new(page: Page, allocator: TinyAllocator) -> TemporaryPage {
		TemporaryPage {
			page,
			allocator,
		}
	}

	/// Maps the temporary page to the given frame in the active table.
	/// Returns the start address of the temporary page.
	pub fn map(&mut self, frame: Frame, active_table: &mut PageMapper) -> VirtualAddress {
		use super::EntryFlags;
		use super::PageLike;

		assert!(active_table.translate(&self.page.start_address()).is_none(), "Temporary page is already mapped");
		active_table.map_to(self.page.clone(), frame, EntryFlags::WRITABLE, &mut self.allocator);
		self.page.start_address()
	}

	/// Un maps the temporary page in the active table without freeing the frame
	pub fn discard(&mut self, active_table: &mut PageMapper) {
		active_table.discard(self.page.clone(), &mut self.allocator);
	}

	/// Maps the temporary page to the given page table frame in the active
	/// table. Returns a reference to the now mapped table.
	pub fn map_table_frame(&mut self, frame: Frame, active_table: &mut PageMapper) -> &mut PageTable<Level1> {
		self.map(frame, active_table);
		unsafe { super::functions::as_table_root(self.page.clone()) }
	}

	pub fn unwrap(self) -> TinyAllocator {
		self.allocator
	}
}
