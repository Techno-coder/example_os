use paging::InactivePageTable;
use paging::VirtualAddress;
use task::Thread;

pub fn load_flat_binary(binary: &[u8], mut base_table: InactivePageTable, entry_point: VirtualAddress) -> Thread {
	use super::functions;
	use paging::EntryFlags;

	// First we copy the binary into memory and map it at the start
	// of the virtual address space
	// Warning: Keep in mind that executing code at address 0 will
	// cause a General Protection Fault so do not set your
	// entry point to be at address 0 (null)
	functions::map_data(binary, &mut base_table, VirtualAddress::new(0),
	                    EntryFlags::USER_ACCESSIBLE);

	// We create stacks at intervals of sixteen pages, so here
	// we calculate the next stack location
	let last_address = binary.len() - 1;
	let stack_bottom = ::utility::math::align_up_usize(last_address, super::stack::STACK_SIZE as usize);
	let stack_bottom = VirtualAddress::new(stack_bottom);
	let (kernel_stack, stack_pointer) = super::stack::create_local_stack(stack_bottom, &mut base_table);

	let stack_data = super::stack::create_initial_stack(&entry_point, &stack_pointer);
	functions::write_data(::utility::convert::as_u8_slice(&stack_data), &mut base_table, stack_pointer.clone());

	Thread {
		page_table: base_table,
		kernel_stack,
		stack_pointer,
	}
}
