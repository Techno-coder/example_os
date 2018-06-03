use super::PageLike;
use super::VirtualAddress;

// This files collates all the special addresses that are used in the kernel
// This makes it easy to see which areas of memory are already used for
// some function

pub const TEMPORARY_PAGE: VirtualAddress = VirtualAddress::new(0xffff_f000_0000_1000);
pub const ACTIVE_TABLE_WITH_TEMPORARY_PAGE: VirtualAddress = TEMPORARY_PAGE.offset(0x1000);
pub const CLONE_SHALLOW_TEMPORARY_PAGE: VirtualAddress = ACTIVE_TABLE_WITH_TEMPORARY_PAGE.offset(0x1000);
pub const HUGE_TEMPORARY_PAGE: VirtualAddress = VirtualAddress::new(0xffff_f000_1000_0000);

pub const HEAP_SIZE: usize = 0x0100_0000_0000 - 1;
pub const HEAP_BOTTOM: VirtualAddress = VirtualAddress::new(0xffff_f100_0000_0000);
pub const HEAP_TOP: VirtualAddress = HEAP_BOTTOM.offset(HEAP_SIZE);

pub const FRAME_STORE_SIZE: usize = 0x0080_0000_0000 - 1;
pub const FRAME_STORE_BOTTOM: VirtualAddress = HEAP_TOP.offset(1);
pub const FRAME_STORE_TOP: VirtualAddress = FRAME_STORE_BOTTOM.offset(FRAME_STORE_SIZE);
pub const HUGE_FRAME_STORE_BOTTOM: VirtualAddress = FRAME_STORE_TOP.offset(1);
pub const HUGE_FRAME_STORE_TOP: VirtualAddress = HUGE_FRAME_STORE_BOTTOM.offset(FRAME_STORE_SIZE);

pub const TASK_SWITCH_STACK_SIZE: usize = super::Page::SIZE as usize - 1;
pub const TASK_SWITCH_STACK_BOTTOM: VirtualAddress = HUGE_FRAME_STORE_TOP.offset(1);
pub const TASK_SWITCH_STACK_TOP: VirtualAddress = TASK_SWITCH_STACK_BOTTOM.offset(TASK_SWITCH_STACK_SIZE);
