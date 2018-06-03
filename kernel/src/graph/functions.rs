use alloc::Vec;
use super::providers::Root;
use utility::Global;

pub static ROOT_PROVIDER: Global<Root> = Global::new("ROOT_PROVIDER");

pub fn load_boot_disk(boot_information: &::multiboot2::BootInformation) {
	use super::Identifier;
	use super::providers::MemoryDisk;
	const MEMORY_DISK_NAME: &'static str = "boot_disk";
	let mut status = ::display::text_mode::BootStatus::new("Loading boot memory disk");

	let module = boot_information.module_tags()
	                             .find(|module| module.name() == MEMORY_DISK_NAME);
	let module = match module {
		Some(module) => module,
		None => {
			status.set_warning().with_message();
			println!("Unable to locate boot memory disk");
			return;
		}
	};

	let data = load_module(module);
	let boot_disk = match MemoryDisk::parse_archive(&data) {
		Some(boot_disk) => boot_disk,
		None => {
			status.set_failure().with_message();
			eprintln!("Failed to parse boot memory disk image");
			return;
		}
	};
	ROOT_PROVIDER.set(Root::new());
	ROOT_PROVIDER.lock().mount(Identifier::new("boot_disk"), box boot_disk);
}

pub fn load_module(module: &::multiboot2::ModuleTag) -> Vec<u8> {
	use memory::FRAME_ALLOCATOR;
	use memory::PhysicalAddress;
	use memory::HugeFrame;
	use memory::FrameLike;
	use paging::HugePage;
	use paging::EntryFlags;
	use paging::PageLike;
	use paging::ACTIVE_PAGE_TABLE;
	use core::ops::DerefMut;

	let start = HugeFrame::from_address(PhysicalAddress::new(module.start_address() as u64));
	let end = HugeFrame::from_address(PhysicalAddress::new(module.end_address() as u64));
	let page = HugePage::from_address(::paging::reserved::HUGE_TEMPORARY_PAGE);

	let mut data = Vec::new();
	for frame in ::memory::FrameIter::inclusive(start, end) {
		let mut current = frame.start_address().raw().max(module.start_address() as u64);
		let end = frame.end_address().raw().min(module.end_address() as u64);

		ACTIVE_PAGE_TABLE.lock().map_to(page.clone(), frame, EntryFlags::empty(), FRAME_ALLOCATOR.lock().deref_mut());
		while current <= end {
			let byte = ((current % HugePage::SIZE) + page.start_address().raw() as u64) as *const u8;
			data.push(unsafe { *byte });
			current += 1;
		}
		ACTIVE_PAGE_TABLE.lock().discard(page.clone(), FRAME_ALLOCATOR.lock().deref_mut());
	}
	data
}
