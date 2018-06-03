pub fn system_call_hook(code: u64) {
	if code == 0xdeadbeef {
		println!("First");
	} else {
		eprintln!("Second {:#x}", code)
	}
}
