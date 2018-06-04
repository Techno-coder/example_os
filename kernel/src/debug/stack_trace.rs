use alloc::BTreeMap;
use paging::VirtualAddress;
use rustc_demangle::Demangle;

pub extern fn stack_trace() {
	// When force-frame-pointers is enabled, every function
	// has these instructions at the start:
	//
	// push rbp
	// mov rbp, rsp
	//
	// When the call instruction is used, the return address
	// (which contains the next instruction to execute when
	// the called function has returned) is pushed onto the stack.
	// Thus, the return address is directly above (because
	// the stack grows downwards) the pushed base pointer.
	// We can use this to create a stack trace.

	println!("Stack trace:");
	let mut base_pointer: *const usize;

	// Get the address of pushed base pointer
	unsafe { asm!("mov rax, rbp" : "={rax}"(base_pointer) ::: "intel") }

	let symbols = super::symbols::SYMBOL_TABLE.try();

	// Before entering boot_entry we set the base pointer to null (0)
	// This way, we can determine when to stop walking the stack
	// See the start64_2 function in boot_entry.asm
	while !base_pointer.is_null() {

		// The return address is above the pushed base pointer
		let return_address = unsafe { *(base_pointer.offset(1)) } as usize;
		let return_address = VirtualAddress::new(return_address);

		// If we haven't loaded the symbol table yet just
		// print the raw return address
		if let Some(symbols) = symbols {
			show_function_call(return_address, symbols);
		} else {
			println!("    Call site: {:#?}", return_address);
		}

		// The pushed base pointer is the address to the previous stack frame
		base_pointer = unsafe { (*base_pointer) as *const usize };
	}
}

fn show_function_call(address: VirtualAddress, symbols: &BTreeMap<VirtualAddress, Demangle>) {
	// The address of every instruction in a function is
	// after the address of the function itself. Thus,
	// we find the symbol with the greatest address that's
	// lower than the return address

	let mut range = symbols.range(..address.clone());
	if let Some((_, identifier)) = range.next_back() {
		println!("    Call site: {:#?}", identifier);
	} else {
		println!("    Call site: {:#?}", address);
	}
}
