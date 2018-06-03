use memory::Frame;
use memory::frame_allocators::TinyAllocator;
use memory::FrameLikeAllocator;
use super::EntryFlags;
use super::Page;
use super::PageLike;
use super::PageMapper;
use super::TemporaryPage;

#[derive(Debug, Clone)]
pub struct InactivePageTable {
	table_root: Frame,
}

impl InactivePageTable {
	pub fn new_cleared(table_root: Frame, page_mapper: &mut PageMapper,
	                   allocator: &mut FrameLikeAllocator<Frame>) -> InactivePageTable {
		let tiny_allocator = TinyAllocator::new(allocator);
		let reserved_page = Page::from_address(super::reserved::TEMPORARY_PAGE);
		let mut temporary_page = TemporaryPage::new(reserved_page, tiny_allocator);

		{
			let table = temporary_page.map_table_frame(table_root.clone(), page_mapper);
			table.clear();
			table[511].set(table_root.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
		}
		temporary_page.discard(page_mapper);
		unsafe { InactivePageTable::new_mapped(table_root) }
	}

	pub fn clone_shallow(&self, page_mapper: &mut PageMapper,
	                     allocator: &mut FrameLikeAllocator<Frame>) -> InactivePageTable {
		use super::PageTable;
		use super::table_level::Level4;

		let original_page = Page::from_address(super::reserved::CLONE_SHALLOW_TEMPORARY_PAGE);
		page_mapper.map_to(original_page.clone(), self.table_root.clone(), EntryFlags::empty(), allocator);

		let table_frame = allocator.allocate().expect("Out of memory: PAGE_TABLE_SHALLOW_CLONE");
		let clone_page = Page::from_address(super::reserved::TEMPORARY_PAGE);
		page_mapper.map_to(clone_page.clone(), table_frame.clone(), EntryFlags::WRITABLE, allocator);

		unsafe {
			let original_table = original_page.start_address().raw() as *const PageTable<Level4>;
			let clone_table = clone_page.start_address().raw() as *mut PageTable<Level4>;
			::core::ptr::copy(original_table, clone_table, 1);
		}

		let table = unsafe { super::functions::as_table_root(clone_page.clone()) };
		table[511].set(table_frame.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);

		page_mapper.discard(original_page, allocator);
		page_mapper.discard(clone_page, allocator);
		unsafe { InactivePageTable::new_mapped(table_frame) }
	}

	/// Creates an interface over an existing PageTable
	///
	/// # Safety
	///
	/// The PageTable must be recursively mapped
	///
	pub unsafe fn new_mapped(table_root: Frame) -> InactivePageTable {
		InactivePageTable {
			table_root,
		}
	}

	pub fn table_root(&self) -> &Frame {
		&self.table_root
	}
}
