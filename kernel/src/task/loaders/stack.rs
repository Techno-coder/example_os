use paging::InactivePageTable;
use paging::Page;
use paging::PageLike;
use paging::VirtualAddress;

#[cfg(debug_assertions)]
pub const PADDING_COUNT: usize = 16;

#[cfg(not(debug_assertions))]
pub const PADDING_COUNT: usize = 15;

pub const EXCEPTION_FRAME_SIZE: usize = 5;
pub const INITIAL_STACK_SIZE: usize = EXCEPTION_FRAME_SIZE + PADDING_COUNT;

pub const STACK_SIZE: u64 = 16 * Page::SIZE;

pub fn create_initial_stack(entry_point: &VirtualAddress, stack_pointer: &VirtualAddress)
                            -> [u64; INITIAL_STACK_SIZE] {
	const R_FLAGS: u64 = 0b10_0000_0010;
	let mut stack = [0; INITIAL_STACK_SIZE];
	stack[PADDING_COUNT] = entry_point.raw() as u64;
	stack[PADDING_COUNT + 1] = *::interrupts::functions::USER_CODE_SELECTOR.try().unwrap() as u64;
	stack[PADDING_COUNT + 2] = R_FLAGS;
	stack[PADDING_COUNT + 3] = stack_pointer.raw() as u64;
	stack[PADDING_COUNT + 4] = *::interrupts::functions::USER_DATA_SELECTOR.try().unwrap() as u64;
	stack
}

pub fn create_local_stack(stack_bottom: VirtualAddress, table: &mut InactivePageTable)
                          -> (Page, VirtualAddress) {
	use paging::PageIter;
	use paging::EntryFlags;
	assert_eq!(stack_bottom.raw() as u64 % STACK_SIZE, 0);

	let kernel_stack_top = stack_bottom.offset(STACK_SIZE as usize - 1);
	let kernel_stack_page = Page::from_address(kernel_stack_top.clone());
	super::functions::allocate_region(PageIter::inclusive(kernel_stack_page.clone(), kernel_stack_page.clone()),
	                                  table, EntryFlags::WRITABLE);

	let stack_top = VirtualAddress::new(kernel_stack_top.raw() - Page::SIZE as usize);
	let stack_bottom_page = Page::from_address(stack_bottom);
	let stack_top_page = Page::from_address(stack_top.clone());
	super::functions::allocate_region(PageIter::inclusive(stack_bottom_page, stack_top_page),
	                                  table, EntryFlags::USER_ACCESSIBLE | EntryFlags::WRITABLE);
	(kernel_stack_page, stack_top)
}