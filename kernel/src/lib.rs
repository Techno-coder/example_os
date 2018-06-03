#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(box_syntax)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate bitflags;
extern crate linked_list_allocator;
extern crate multiboot2;
extern crate rlibc;
extern crate rustc_demangle;
extern crate spin;
extern crate x86_64;

use memory::functions::HEAP_ALLOCATOR;

#[macro_use]
mod display;
#[cfg(test)]
mod tests;
mod debug;
mod interrupts;
mod structures;
mod memory;
mod paging;
mod utility;
mod keyboard;
mod shell;
mod graph;
mod system_call;
mod task;

pub const KERNEL_BASE: u64 = 0xffff_ff00_0000_0000;

#[no_mangle]
pub extern "C" fn boot_entry(boot_information: usize) -> ! {
	//  This function is called after jumping from start64_2 in boot_entry.asm

	let boot_structure = ::utility::MultibootStructure::new(boot_information as usize);
	let boot_information = boot_structure.get();
	::display::text_mode::functions::initialize();
	::interrupts::functions::initialize();

	let mut boot_allocator = ::memory::functions::initialize(boot_structure.clone());
	let base_table = ::paging::functions::initialize(&boot_information, &mut boot_allocator);
	::memory::functions::post_paging_initialize(boot_allocator);
	::graph::functions::load_boot_disk(&boot_information);
	::debug::symbols::load_kernel_symbols();
	::memory::functions::post_initialize(&boot_information);

	::task::functions::pre_initialize();
	::task::functions::initialize();
	::shell::functions::initialize();
	::interrupts::functions::post_initialize();

	println!("Kernel boot successful");
	println!("Press any key to launch the kernel shell");
	loop { unsafe { ::x86_64::instructions::halt(); } }
}

#[lang = "eh_personality"]
#[cfg(not(test))]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[cfg(not(test))]
#[no_mangle]
pub extern fn panic_fmt(fmt: ::core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
	eprintln!("\nKernel Panic in");
	eprintln!("    {}", file);
	eprintln!("at line {} column {}", line, column);
	eprintln!("    {}", fmt);
	::debug::stack_trace();
	loop { unsafe { ::x86_64::instructions::halt(); } }
}

#[lang = "oom"]
#[cfg(not(test))]
#[no_mangle]
pub extern fn oom() -> ! {
	panic!("Out of memory");
}
