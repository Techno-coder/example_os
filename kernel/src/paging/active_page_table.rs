use memory::Frame;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use memory::PhysicalAddress;
use super::InactivePageTable;
use super::PageMapper;
use super::TemporaryPage;

pub struct ActivePageTable {
	page_mapper: PageMapper,
}

impl ActivePageTable {
	/// Creates an interface over the current page table
	///
	/// # Safety
	///
	/// Only one `ActivePageTable` instance should exist
	///
	pub unsafe fn new() -> ActivePageTable {
		ActivePageTable {
			page_mapper: PageMapper::new(),
		}
	}

	pub fn with<F, R>(&mut self, inactive_table: &mut InactivePageTable,
	                  allocator: &mut FrameLikeAllocator<Frame>, f: F) -> R
		where F: FnOnce(&mut PageMapper, &mut FrameLikeAllocator<Frame>) -> R {
		use super::EntryFlags;
		use super::PageLike;
		use memory::frame_allocators::TinyAllocator;
		use x86_64::instructions::tlb::flush_all;

		let tiny_allocator = TinyAllocator::new(allocator);
		let page = super::Page::from_address(::paging::reserved::ACTIVE_TABLE_WITH_TEMPORARY_PAGE);
		let mut temporary_page = TemporaryPage::new(page, tiny_allocator);
		let active_table_frame = Frame::from_address(self.current_table_address());

		let value;
		{
			let active_table = temporary_page.map_table_frame(active_table_frame.clone(), self);

			self.table_mut()[511].set(inactive_table.table_root().clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
			flush_all();
			value = f(self, allocator);

			active_table[511].set(active_table_frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
			flush_all();
		}

		temporary_page.discard(self);
		temporary_page.unwrap().dispose(allocator);
		value
	}

	pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
		let current_table_frame = Frame::from_address(self.current_table_address());
		let current_table = unsafe { InactivePageTable::new_mapped(current_table_frame) };
		unsafe {
			let new_table_address = new_table.table_root().start_address().raw();
			let new_table_address = ::x86_64::PhysicalAddress(new_table_address);
			::x86_64::registers::control_regs::cr3_write(new_table_address);
		}
		current_table
	}

	pub fn current_table_address(&self) -> PhysicalAddress {
		let active_table_address = ::x86_64::registers::control_regs::cr3();
		PhysicalAddress::new(active_table_address.0)
	}
}

impl ::core::ops::Deref for ActivePageTable {
	type Target = PageMapper;

	fn deref(&self) -> &Self::Target {
		&self.page_mapper
	}
}

impl ::core::ops::DerefMut for ActivePageTable {
	fn deref_mut(&mut self) -> &mut PageMapper {
		&mut self.page_mapper
	}
}