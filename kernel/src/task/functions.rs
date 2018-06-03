use alloc::boxed::Box;
use paging::VirtualAddress;
use super::Scheduler;
use super::Thread;
use utility::Global;

pub static ACTIVE_THREAD: Global<Thread> = Global::new("ACTIVE_THREAD");
pub static SCHEDULER: Global<Box<Scheduler + Send>> = Global::new("SCHEDULER");

pub fn initialize() {
	let _status = ::display::text_mode::BootStatus::new("Creating preemptive scheduler");
	enable_cpu_features();

	let scheduler = super::schedulers::RoundRobin::new();
	SCHEDULER.set(box scheduler);
}

pub fn pre_initialize() {
	use paging::Page;
	use paging::PageLike;
	use memory::FrameLikeAllocator;
	use core::ops::DerefMut;

	let _status = ::display::text_mode::BootStatus::new("Allocating context switch stacks");
	let mut active_table = ::paging::ACTIVE_PAGE_TABLE.lock();
	let mut allocator = ::memory::FRAME_ALLOCATOR.lock();

	// We use a separate stack when facilitating a context switch
	// See interrupts/handlers::timer_handler
	let stack_start = Page::from_address(::paging::reserved::TASK_SWITCH_STACK_BOTTOM);
	let stack_end = Page::from_address(::paging::reserved::TASK_SWITCH_STACK_TOP);
	let pages = ::paging::PageIter::inclusive(stack_start, stack_end);
	for page in pages {
		let frame = allocator.allocate().expect("Out of memory: CONTEXT_SWITCH_STACK_ALLOCATION");
		active_table.map_to(page, frame, ::paging::EntryFlags::WRITABLE, allocator.deref_mut());
	}
}

fn enable_cpu_features() {
	use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};
	const SCE_BIT: u64 = 1;
	unsafe {
		let efer = rdmsr(IA32_EFER);
		wrmsr(IA32_EFER, efer | SCE_BIT);
	}
}

pub extern "C" fn context_switch(stack_pointer: usize) -> usize {
	use core::ops::DerefMut;
	use paging::PageLike;

	let mut scheduler = SCHEDULER.lock();
	let mut active_thread = ACTIVE_THREAD.lock_direct();
	let mut active_table = ::paging::ACTIVE_PAGE_TABLE.lock();

	// If this is our first context_switch, then there won't be
	// an active thread
	if let Some(mut thread) = active_thread.take() {
		thread.stack_pointer = VirtualAddress::new(stack_pointer);
		scheduler.schedule_new(thread);
	}

	let new_thread: Thread = scheduler.schedule_next().expect("Scheduler is empty");
	::core::mem::replace(active_thread.deref_mut(), Some(new_thread.clone()));

	// This stack is switched to whenever an interrupt occurs in user mode
	// A kernel stack is needed to facilitate system calls and timer interrupts
	let kernel_stack = ::x86_64::VirtualAddress(new_thread.kernel_stack.end_address().raw());
	::interrupts::functions::TSS.lock().privilege_stack_table[0] = kernel_stack;

	// Switching page tables invalidates the previous kernel stack so
	// that's why we use a separate stack for handling the context switch
	active_table.switch(new_thread.page_table);

	::interrupts::send_interrupt_end(false);
	new_thread.stack_pointer.raw()
}
