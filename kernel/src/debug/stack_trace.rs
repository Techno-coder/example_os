use alloc::BTreeMap;
use paging::VirtualAddress;
use rustc_demangle::Demangle;

pub extern fn stack_trace() {
	println!("Stack trace:");
	let mut base_pointer: *const usize;
	unsafe { asm!("mov rax, rbp" : "={rax}"(base_pointer) ::: "intel") }

	let symbols = super::symbols::SYMBOL_TABLE.try();
	while !base_pointer.is_null() {
		let return_address = unsafe { *(base_pointer.offset(1)) } as usize;
		let return_address = VirtualAddress::new(return_address);
		if let Some(symbols) = symbols {
			show_function_call(return_address, symbols);
		} else {
			println!("    Call site: {:#?}", return_address);
		}
		base_pointer = unsafe { (*base_pointer) as *const usize };
	}
}

fn show_function_call(address: VirtualAddress, symbols: &BTreeMap<VirtualAddress, Demangle>) {
	let mut range = symbols.range(..address.clone());
	if let Some((_, identifier)) = range.next_back() {
		println!("    Call site: {:#?}", identifier);
	} else {
		println!("    Call site: {:#?}", address);
	}
}
