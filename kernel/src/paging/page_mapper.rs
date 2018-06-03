use memory::Frame;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use memory::HugeFrame;
use memory::PhysicalAddress;
use super::EntryFlags;
use super::Page;
use super::PageLike;
use super::PageTable;
use super::table_level::Level4;
use super::VirtualAddress;

pub struct PageMapper {
	table_root: &'static mut PageTable<Level4>,
}

impl PageMapper {
	const TABLE_4_ADDRESS: u64 = 0xffff_ffff_ffff_f000;

	/// Creates an interface over the level four page table
	///
	/// # Safety
	///
	/// The page table should be recursively mapped
	/// Only one `PageMapper` instance should exist
	///
	pub unsafe fn new() -> PageMapper {
		PageMapper {
			table_root: &mut *((PageMapper::TABLE_4_ADDRESS) as *mut PageTable<Level4>),
		}
	}

	pub fn table(&self) -> &PageTable<Level4> {
		self.table_root
	}

	pub fn table_mut(&mut self) -> &mut PageTable<Level4> {
		self.table_root
	}

	pub fn map_to<P>(&mut self, page: P, frame: P::FrameType, flags: EntryFlags,
	                 allocator: &mut FrameLikeAllocator<Frame>) where P: PageLike {
		page.map_to(self, frame, flags | EntryFlags::PRESENT, allocator);
	}

	#[must_use]
	pub fn un_map<P>(&mut self, page: P, allocator: &mut FrameLikeAllocator<Frame>)
	                 -> P::FrameType where P: PageLike {
		let frame = page.un_map(self, allocator);
		self.flush_table_entry(&page);
		frame
	}

	pub fn discard<P>(&mut self, page: P, allocator: &mut FrameLikeAllocator<Frame>) where P: PageLike {
		let _ = self.un_map(page, allocator);
	}

	pub fn translate(&self, address: &VirtualAddress) -> Option<PhysicalAddress> {
		let page = Page::from_address(address.clone());
		let table_2 = self.table().next_table(page.table_4_index())
		                  .and_then(|table_3| table_3.next_table(page.table_3_index()))?;

		if table_2[page.table_2_index()].flags().contains(EntryFlags::HUGE_PAGE) {
			let huge_frame: HugeFrame = table_2[page.table_2_index()].frame()?;
			let raw_address = huge_frame.start_address().raw() + (address.raw() as u64 % HugeFrame::SIZE);
			return Some(PhysicalAddress::new(raw_address));
		}

		let frame: Frame = table_2.next_table(page.table_2_index())
		                          .and_then(|table_1| table_1[page.table_1_index()].frame())?;
		let raw_address = frame.start_address().raw() + (address.raw() as u64 % Frame::SIZE);
		Some(PhysicalAddress::new(raw_address))
	}

	pub fn flush_table_entry<P>(&mut self, page: &P) where P: PageLike {
		use x86_64::VirtualAddress;
		::x86_64::instructions::tlb::flush(VirtualAddress(page.start_address().raw()));
	}
}
