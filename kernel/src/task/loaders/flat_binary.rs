use paging::InactivePageTable;
use paging::VirtualAddress;
use task::Thread;

pub fn load_flat_binary(binary: &[u8], mut base_table: InactivePageTable, entry_point: VirtualAddress) -> Thread {
	use super::functions;
	use paging::EntryFlags;
	functions::map_data(binary, &mut base_table, VirtualAddress::new(0),
	                    EntryFlags::USER_ACCESSIBLE);

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
