use super::send_interrupt_end;
use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

pub extern "x86-interrupt" fn zero_divide_handler(stack_frame: &mut ExceptionStackFrame) {
	panic!("\nDivide by zero: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
	panic!("\nBreakpoint: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
	panic!("\nDouble Fault: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
	// The cr2 register contains the address that caused the
	// page fault when a page fault occurs
	let address = ::x86_64::registers::control_regs::cr2();
	let address = ::paging::VirtualAddress::new(address.0);
	if !::memory::functions::handle_heap_fault(address, &error_code) {
		panic!("\nPage Fault: {:#?}\n{:#?}", error_code, stack_frame);
	}
}

pub extern "x86-interrupt" fn general_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
	panic!("\nGeneral Protection Fault: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame) {
	panic!("\nInvalid Opcode Fault: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut ExceptionStackFrame) {
	// Note: This handler isn't actually used because we've
	// disabled the keyboard interrupt. See interrupts::pic_functions
	if let Some(key_code) = ::keyboard::SYSTEM_KEYBOARD.lock().parse_port_input() {
		::shell::SYSTEM_SHELL.lock().on_key_press(key_code);
	}
	send_interrupt_end(false);
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut ExceptionStackFrame) {
	const TASK_SWITCH_STACK_TOP: usize = ::paging::reserved::TASK_SWITCH_STACK_TOP.raw();
	// All the registers are pushed here
	unsafe {
		// First, we save the current stack pointer so we can store it in the scheduler later.
		// We also switch stacks because if we don't, when we change page tables
		// the stack will not longer be valid, which causes a loop of page faults.
		// The first argument to a function is stored in the rdi register
		asm!("mov rdi, rsp
			  mov rsp, $0
			  call $1
			  mov rsp, rax"
			  :: "i"(TASK_SWITCH_STACK_TOP), // i indicates that the argument is a constant
			  "i"(::task::functions::context_switch as extern "C" fn(usize) -> usize)
			  : "rax","rbx","rcx","rdx","rbp","rsi","rdi","r8","r9","r10","r11","r12","r13","r14","r15"
			  : "intel");
		// All registers are clobbered to force LLVM to push and pop all the registers
		// onto the stack, thus saving them.
		// If the user mode threads use floating operations too, then
		// the floating point registers need to be pushed and popped as well
		//
		// Note: When the context switch happens, all the registers are still on the stack
		// and are popped when the context switch comes back to this stack
	}
	// All the registers are popped here
}

pub extern "x86-interrupt" fn system_call_handler(_stack_frame: &mut ExceptionStackFrame) {
	let code;
	// TODO create real system call handler
	unsafe {
		// The number in r15 is moved into the `code` variable
		asm!("mov rbx, r15" : "={rbx}"(code) :: "rbx", "r15" : "intel");
	}
	::system_call::functions::system_call_hook(code);
}
