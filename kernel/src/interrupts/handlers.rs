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
	if let Some(key_code) = ::keyboard::SYSTEM_KEYBOARD.lock().parse_port_input() {
		::shell::SYSTEM_SHELL.lock().on_key_press(key_code);
	}
	send_interrupt_end(false);
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut ExceptionStackFrame) {
	const TASK_SWITCH_STACK_TOP: usize = ::paging::reserved::TASK_SWITCH_STACK_TOP.raw();
	unsafe {
		asm!("mov rdi, rsp
			  mov rsp, $0
			  call $1
			  mov rsp, rax"
			  :: "i"(TASK_SWITCH_STACK_TOP),
			  "i"(::task::functions::context_switch as extern "C" fn(usize) -> usize)
			  : "rax","rbx","rcx","rdx","rbp","rsi","rdi","r8","r9","r10","r11","r12","r13","r14","r15"
			  : "intel");
	}
}

pub extern "x86-interrupt" fn system_call_handler(_stack_frame: &mut ExceptionStackFrame) {
	let code;
	// TODO create real system call handler
	unsafe {
		asm!("mov rbx, r15" : "={rbx}"(code) :: "rbx", "r15" : "intel");
	}
	::system_call::functions::system_call_hook(code);
}
