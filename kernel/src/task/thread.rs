use paging::InactivePageTable;
use paging::Page;
use paging::VirtualAddress;

#[derive(Debug, Clone)]
pub struct Thread {
	pub page_table: InactivePageTable,
	pub kernel_stack: Page,
	pub stack_pointer: VirtualAddress,
}

impl Thread {}