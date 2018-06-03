use core::ops::DerefMut;
use memory::Frame;
use memory::FRAME_ALLOCATOR;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use paging::ACTIVE_PAGE_TABLE;
use paging::ActivePageTable;
use paging::EntryFlags;
use paging::InactivePageTable;
use paging::Page;
use paging::PageIter;
use paging::PageLike;
use paging::VirtualAddress;

pub fn map_data(data: &[u8], table: &mut InactivePageTable, offset: VirtualAddress, flags: EntryFlags) {
	let end_address = offset.offset(data.len() - 1);
	let start_page = Page::from_address(offset.clone());
	let end_page = Page::from_address(end_address);
	allocate_region(PageIter::inclusive(start_page, end_page), table, flags);
	write_data(data, table, offset);
}

pub fn allocate_region(region: PageIter<Page>, table: &mut InactivePageTable, flags: EntryFlags) {
	let mut allocator = FRAME_ALLOCATOR.lock();
	let allocator = allocator.deref_mut();
	for page in region {
		let frame = allocator.allocate().expect("Out of memory: REGION_ALLOCATION");
		ACTIVE_PAGE_TABLE.lock().with(table, allocator, |table, allocator| {
			table.map_to(page, frame, flags.clone(), allocator);
		});
	}
}

pub fn write_data(data: &[u8], table: &mut InactivePageTable, offset: VirtualAddress) {
	let staging_page = Page::from_address(::paging::reserved::TEMPORARY_PAGE);
	let mut active_table = ACTIVE_PAGE_TABLE.lock();
	let mut allocator = FRAME_ALLOCATOR.lock();
	let allocator = allocator.deref_mut();

	let mut map_frame = |active_table: &mut ActivePageTable, allocator: &mut FrameLikeAllocator<Frame>,
	                     address: &VirtualAddress| {
		let frame = active_table.with(table, allocator, |table, _| {
			Frame::from_address(table.translate(address).expect("Data destination not mapped"))
		});
		active_table.map_to(staging_page.clone(), frame, EntryFlags::WRITABLE, allocator);
	};

	map_frame(&mut active_table, allocator, &offset);
	for (index, byte) in data.iter().enumerate() {
		let destination_address = offset.offset(index);
		let on_page_boundary = destination_address.raw() as u64 % Page::SIZE == 0;
		if on_page_boundary && index != 0 {
			active_table.discard(staging_page.clone(), allocator);
			map_frame(&mut active_table, allocator, &destination_address);
		}

		let byte_index = (destination_address.raw() as u64 % Page::SIZE) as usize;
		let destination_byte = (staging_page.start_address().raw() + byte_index) as *mut u8;
		unsafe { *destination_byte = *byte; }
	}
	active_table.discard(staging_page.clone(), allocator);
}
