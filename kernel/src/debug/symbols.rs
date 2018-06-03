use alloc::BTreeMap;
use alloc::String;
use alloc::Vec;
use graph::Provider;
use paging::VirtualAddress;
use rustc_demangle::Demangle;
use spin::Once;

// Demangle stores a reference to a string so the
// symbol table string has to have a static lifetime
static TABLE_STRING: Once<String> = Once::new();
pub static SYMBOL_TABLE: Once<BTreeMap<VirtualAddress, Demangle>> = Once::new();

pub fn load_kernel_symbols() {
	if let Some(string) = load_symbol_table() {
		let string = TABLE_STRING.call_once(|| string);
		SYMBOL_TABLE.call_once(|| parse_symbols(string));
	}
}

pub fn load_symbol_table() -> Option<String> {
	macro_rules! table_location { () => { "kernel/symbols.table" }; }
	let mut status = ::display::text_mode::BootStatus::new("Loading kernel debug symbols");

	let symbol_table_path = ::graph::Location::parse(concat!("boot_disk/", table_location!()));

	// Load the symbol table file from our boot disk
	let symbol_table = ::graph::ROOT_PROVIDER.lock().open(&symbol_table_path.as_slice());
	let symbol_table = if let Some(mut table) = symbol_table {
		table.read_all()
	} else {
		status.set_warning().with_message();
		println!("Missing {} file in boot disk", table_location!());
		return None;
	};

	// Copy the file data as a string for easier parsing
	let symbol_table = String::from_utf8(symbol_table);
	let table_string = if let Ok(symbol_table) = symbol_table {
		symbol_table
	} else {
		status.set_failure().with_message();
		eprintln!("Failed to parse symbol table string");
		return None;
	};
	Some(table_string)
}

pub fn parse_symbols(table_string: &str) -> BTreeMap<VirtualAddress, Demangle> {
	let mut status = ::display::text_mode::BootStatus::new("Demangling kernel symbol identifiers");
	let mut map = BTreeMap::new();
	let mut skipped_count = 0;
	let mut parsed_count = 0;

	let entries = table_string.split('\n');
	for entry in entries {
		let segments: Vec<&str> = entry.split_whitespace().collect();

		// Each line in the symbol table must has three columns
		// because we use the first and third columns
		if segments.len() < 3 {
			skipped_count += 1;
			continue;
		}

		// The first column contains the address of the symbol,
		// but it is in a hexadecimal string form so we have to
		// convert it to a number
		let address = usize::from_str_radix(segments[0], 16);
		let address = if let Ok(address) = address {
			VirtualAddress::new(address)
		} else {
			skipped_count += 1;
			continue;
		};

		// The third column contains the name of the symbol,
		// but it has been mangled by the Rust compiler. We
		// demangle it here.
		let identifier = ::rustc_demangle::demangle(segments[2]);
		map.insert(address, identifier);
		parsed_count += 1;
	}

	status.set_success().with_message();
	println!("Successfully parsed {} out of {} entries", parsed_count, parsed_count + skipped_count);
	map
}