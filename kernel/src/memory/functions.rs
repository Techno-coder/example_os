use core::ops::DerefMut;
use linked_list_allocator::LockedHeap;
use memory::FrameLikeAllocator;
use paging::EntryFlags;
use paging::PageLike;
use paging::reserved::HEAP_BOTTOM;
use paging::VirtualAddress;
use super::generic_allocators::BootAllocator;
use super::generic_allocators::GlobalFrameAllocator;
use utility::Global;
use x86_64::structures::idt::PageFaultErrorCode;

#[cfg_attr(not(test), global_allocator)]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
pub static FRAME_ALLOCATOR: Global<GlobalFrameAllocator> = Global::new("FRAME_ALLOCATOR");

pub fn initialize(boot_structure: ::utility::MultibootStructure) -> BootAllocator {
	let _status = ::display::text_mode::BootStatus::new("Creating boot frame allocator");
	BootAllocator::new(boot_structure)
}

pub fn post_paging_initialize(mut allocator: BootAllocator) {
	let _status = ::display::text_mode::BootStatus::new("Initializing kernel heap");
	create_initial_heap(&mut allocator);
	unsafe { HEAP_ALLOCATOR.lock().init(HEAP_BOTTOM.raw(), ::paging::reserved::HEAP_SIZE); }
	FRAME_ALLOCATOR.set(GlobalFrameAllocator::Boot(allocator));
}

pub fn post_initialize(boot_information: &::multiboot2::BootInformation) {
	let _status = ::display::text_mode::BootStatus::new("Creating system frame allocator");

	// We must be careful to not do any heap allocation when
	// converting the BootAllocator. Heap allocation requires
	// access to the BootAllocator but to convert it, we have
	// to lock it. This means if we heap allocate, it will
	// cause a deadlock. See structures/frame_store
	let mut free_areas = ::alloc::Vec::new();
	for area in boot_information.memory_map_tag().unwrap().memory_areas() {
		let memory_area: super::MemoryArea = area.into();
		free_areas.push(memory_area);
	}

	FRAME_ALLOCATOR.lock().convert(free_areas);
}

fn create_initial_heap<A>(allocator: &mut A) where A: ::memory::GenericAllocator {
	let mut table = ::paging::ACTIVE_PAGE_TABLE.lock();
	let page = ::paging::HugePage::from_address(HEAP_BOTTOM.clone());
	let frame = allocator.allocate().expect("Out of memory: Initial heap");
	table.map_to(page, frame, EntryFlags::WRITABLE, allocator);
}

pub fn handle_heap_fault(address: VirtualAddress, _error_code: &PageFaultErrorCode) -> bool {
	// The kernel's heap does not have a predefined size;
	// instead, when an address is in the heap but not
	// allocated, we allocate it here. This allows the
	// heap to grow as large as needed

	if !(address >= HEAP_BOTTOM && address <= ::paging::reserved::HEAP_TOP) {
		return false;
	}

	let page = ::paging::HugePage::from_address(address);

	// Note: We lock the FRAME_ALLOCATOR here which results
	// in a deadlock if it was already locked
	let mut allocator = FRAME_ALLOCATOR.lock();
	let frame = allocator.allocate().expect("Out of memory: Heap fault");

	// Same goes for the ACTIVE_PAGE_TABLE.
	let mut table = ::paging::ACTIVE_PAGE_TABLE.lock();
	table.map_to(page, frame, EntryFlags::WRITABLE, allocator.deref_mut());
	true
}