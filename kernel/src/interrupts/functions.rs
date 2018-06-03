use spin::Once;
use structures::FixedStack;
use super::GdtDescriptor;
use utility::Global;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::idt::Idt;
use x86_64::structures::tss::TaskStateSegment;

static GDT: Once<super::Gdt> = Once::new();
static IDT: Once<Idt> = Once::new();
pub static TSS: Global<TaskStateSegment> = Global::new("TASK_STATE_SEGMENT");

pub static USER_CODE_SELECTOR: Once<u16> = Once::new();
pub static USER_DATA_SELECTOR: Once<u16> = Once::new();

// These stacks must be marked as mutable so they are placed
// in the .bss segment. Otherwise, it would cause a page fault
// because it would be in the .rodata section.
static mut DOUBLE_FAULT_STACK: FixedStack = FixedStack::new();
static mut PAGE_FAULT_STACK: FixedStack = FixedStack::new();
static mut GENERAL_FAULT_STACK: FixedStack = FixedStack::new();

const DOUBLE_FAULT_STACK_INDEX: usize = 0;
const PAGE_FAULT_STACK_INDEX: usize = 2;
const GENERAL_FAULT_STACK_INDEX: usize = 3;

const TIMER_INTERRUPT_INDEX: usize = 0;
const KEYBOARD_INTERRUPT_INDEX: usize = 1;
const SYSTEM_CALL_INDEX: usize = 0xaa - super::pic_functions::PIC_ONE_VECTOR_BASE as usize;

pub fn initialize() {
	let _status = ::display::text_mode::BootStatus::new("Initializing interrupt descriptor table");
	initialize_task_state_segment();
	initialize_global_descriptor_table();
	initialize_interrupt_table();
}

pub fn post_initialize() {
	let _status = ::display::text_mode::BootStatus::new("Enabling interrupts");
	super::pic_functions::initialize();
	super::pit_functions::initialize();

	// Enabling interrupts allows timer interrupts to be
	// fired and handled.
	unsafe { ::x86_64::instructions::interrupts::enable(); }
}

fn initialize_global_descriptor_table() {
	use core::ops::Deref;

	let mut kernel_code_selector = SegmentSelector(0);
	let mut kernel_data_selector = SegmentSelector(0);
	let mut user_code_selector = SegmentSelector(0);
	let mut user_data_selector = SegmentSelector(0);
	let mut tss_selector = SegmentSelector(0);

	let gdt = GDT.call_once(|| {
		let mut gdt = super::Gdt::new();
		kernel_code_selector = gdt.add_kernel_entry(GdtDescriptor::kernel_code_segment());
		kernel_data_selector = gdt.add_kernel_entry(GdtDescriptor::kernel_data_segment());
		user_code_selector = gdt.add_user_entry(GdtDescriptor::user_code_segment());
		user_data_selector = gdt.add_user_entry(GdtDescriptor::user_data_segment());
		tss_selector = gdt.add_kernel_entry(GdtDescriptor::tss_segment(&TSS.lock().deref()));
		gdt
	});
	gdt.load();

	USER_CODE_SELECTOR.call_once(|| user_code_selector.0);
	USER_DATA_SELECTOR.call_once(|| user_data_selector.0);

	unsafe {
		::x86_64::instructions::segmentation::set_cs(kernel_code_selector);
		::x86_64::instructions::segmentation::load_ds(kernel_data_selector);
		::x86_64::instructions::tables::load_tss(tss_selector);
	}
}

fn initialize_task_state_segment() {
	use x86_64::VirtualAddress;
	let mut tss = TaskStateSegment::new();
	unsafe {
		// The TaskStateSegment can store up to seven stacks
		// These stacks are switched to when interrupts handlers
		// are called
		tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX] = VirtualAddress(DOUBLE_FAULT_STACK.address());
		tss.interrupt_stack_table[PAGE_FAULT_STACK_INDEX] = VirtualAddress(PAGE_FAULT_STACK.address());
		tss.interrupt_stack_table[GENERAL_FAULT_STACK_INDEX] = VirtualAddress(GENERAL_FAULT_STACK.address());
	}
	TSS.set(tss);
}

fn initialize_interrupt_table() {
	let mut table = Idt::new();
	unsafe { set_interrupt_handlers(&mut table); }
	IDT.call_once(|| table).load();
}

unsafe fn set_interrupt_handlers(table: &mut Idt) {
	use super::handlers::*;
	use x86_64::PrivilegeLevel;

	// Note: By default, interrupts are automatically disabled
	// by the processor when a interrupt handler is called
	// and enabled when the handler returns
	table.divide_by_zero.set_handler_fn(zero_divide_handler);
	table.double_fault.set_handler_fn(double_fault_handler)
	     .set_stack_index(DOUBLE_FAULT_STACK_INDEX as u16);
	table.breakpoint.set_handler_fn(breakpoint_handler);
	table.page_fault.set_handler_fn(page_fault_handler)
	     .set_stack_index(PAGE_FAULT_STACK_INDEX as u16);
	table.general_protection_fault.set_handler_fn(general_fault_handler)
	     .set_stack_index(GENERAL_FAULT_STACK_INDEX as u16);
	table.invalid_opcode.set_handler_fn(invalid_opcode_handler);
	table.interrupts[TIMER_INTERRUPT_INDEX].set_handler_fn(timer_handler);
	table.interrupts[KEYBOARD_INTERRUPT_INDEX].set_handler_fn(keyboard_handler);

	// We allow interrupts so the scheduler can preempt a system call
	// We need the privilege level to be Ring3 so user mode
	// threads can use
	//
	// int 0xaa
	//
	// without causing a General Protection Fault
	table.interrupts[SYSTEM_CALL_INDEX]
		.set_handler_fn(system_call_handler)
		.disable_interrupts(false)
		.set_privilege_level(PrivilegeLevel::Ring3);
}
