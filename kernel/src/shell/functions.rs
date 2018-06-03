use super::KernelShell;
use utility::Global;

pub static SYSTEM_SHELL: Global<KernelShell> = Global::new("SYSTEM_SHELL");

pub fn initialize() {
	let _status = ::display::text_mode::BootStatus::new("Initializing kernel shell");
	SYSTEM_SHELL.set(KernelShell::new());
}
