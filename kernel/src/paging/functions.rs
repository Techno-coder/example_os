use memory::Frame;
use memory::FrameIter;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use memory::PhysicalAddress;
use multiboot2::BootInformation;
use super::ActivePageTable;
use super::EntryFlags;
use super::InactivePageTable;
use super::Page;
use super::PageLike;
use super::PageMapper;
use super::VirtualAddress;
use utility::Global;

pub static ACTIVE_PAGE_TABLE: Global<ActivePageTable> = Global::new("ACTIVE_PAGE_TABLE");

pub fn initialize<A>(boot_information: &BootInformation, allocator: &mut A) -> InactivePageTable
	where A: FrameLikeAllocator<Frame> {
	let _status = ::display::text_mode::BootStatus::new("Remapping kernel memory sections");

	let _elf_sections_tag = boot_information.elf_sections_tag().expect("ElfSectionsTag required");
	let _memory_map_tag = boot_information.memory_map_tag().expect("MemoryMapTag required");

	enable_cpu_features();
	let base_table = remap_kernel(boot_information, allocator);

	ACTIVE_PAGE_TABLE.set(unsafe { ActivePageTable::new() });
	base_table
}

pub unsafe fn as_table_root<'a>(table_root: Page) -> &'a mut super::PageTable<super::table_level::Level1> {
	&mut *(table_root.start_address().raw() as *mut super::PageTable<super::table_level::Level1>)
}

fn enable_cpu_features() {
	enable_nxe_bit();
	enable_write_protect_bit();
	enable_global_pages_bit();
}

fn enable_nxe_bit() {
	// Allows NO_EXECUTE to be marked on pages

	use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};
	const NXE_BIT: u64 = 1 << 11;
	unsafe {
		let efer = rdmsr(IA32_EFER);
		wrmsr(IA32_EFER, efer | NXE_BIT);
	}
}

fn enable_write_protect_bit() {
	use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};
	unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

fn enable_global_pages_bit() {
	// Global pages aren't actually used anywhere,
	// but they can be used for marking kernel pages
	// to prevent those entries from being flushed
	// on context switches

	use x86_64::registers::control_regs::{cr4, cr4_write, Cr4};
	unsafe { cr4_write(cr4() | Cr4::ENABLE_GLOBAL_PAGES) };
}

fn remap_kernel<A>(boot_information: &BootInformation, allocator: &mut A) -> InactivePageTable
	where A: FrameLikeAllocator<Frame> {
	let mut active_table = unsafe { ActivePageTable::new() };

	let mut remap_table = {
		let frame = allocator.allocate().expect("Out of memory: Kernel remap");
		InactivePageTable::new_cleared(frame, &mut active_table, allocator)
	};

	// We do not map the multiboot sections here because they are
	// not usable once we finish booting
	active_table.with(&mut remap_table, allocator, |mapper, allocator| {
		remap_vga_buffer(mapper, allocator);
		remap_kernel_sections(boot_information, mapper, allocator);
		prepare_kernel_page_directories(mapper, allocator);
	});
	let base_table = remap_table.clone_shallow(&mut active_table, allocator);

	active_table.with(&mut remap_table, allocator, |mapper, allocator| {
		remap_multiboot(boot_information, mapper, allocator);
	});

	// We switch to the newly created page table
	let old_table = active_table.switch(remap_table);
	create_guard_page(old_table, &mut active_table, allocator);
	base_table
}

fn create_guard_page<A>(old_table: InactivePageTable, mapper: &mut PageMapper, allocator: &mut A)
	where A: FrameLikeAllocator<Frame> {
	let old_table_raw_address = old_table.table_root().start_address().raw();
	let old_table_address = VirtualAddress::new_adjusted(old_table_raw_address as usize);

	// The old page table is directly located below our kernel stack
	// Thus, if we un map the old page table, if we overflow our
	// stack, a page fault will occur.
	let guard_page = Page::from_address(old_table_address);
	let frame = mapper.un_map(guard_page, allocator);
	allocator.deallocate(frame);
}

fn remap_kernel_sections(boot_info: &BootInformation, mapper: &mut PageMapper, allocator: &mut FrameLikeAllocator<Frame>) {
	let elf_sections = boot_info.elf_sections_tag().unwrap();
	for section in elf_sections.sections() {
		// If the section isn't allocated, then it doesn't exist in memory
		if !section.is_allocated() {
			continue;
		}

		let flags = EntryFlags::kernel_elf_section(&section);

		// For this to work correctly, all kernel sections have to be aligned
		// on a page boundary. To use a HugeFrame, all sections have to be
		// aligned on a HugeFrame boundary. Since the kernel is so small,
		// it is not worth using HugeFrames. See linker.ld
		let start_frame = Frame::from_address(PhysicalAddress::new_adjusted(section.start_address()));
		let end_frame = Frame::from_address(PhysicalAddress::new_adjusted(section.end_address() - 1));

		for frame in FrameIter::inclusive_unchecked(start_frame, end_frame) {
			let page = Page::from_address(VirtualAddress::new_adjusted(frame.start_address().raw() as usize));
			mapper.map_to(page, frame, flags, allocator);
		}
	}
}

fn remap_vga_buffer(mapper: &mut PageMapper, allocator: &mut FrameLikeAllocator<Frame>) {
	// We access the VGA buffer using a higher half kernel page
	let vga_buffer_page = Page::from_address(VirtualAddress::new_adjusted(0xb8000));

	// However, the actual VGA buffer is still located in the lower half
	// of physical memory
	let vga_buffer_frame = Frame::from_address(PhysicalAddress::new(0xb8000));
	mapper.map_to(vga_buffer_page, vga_buffer_frame, EntryFlags::WRITABLE, allocator);
}

fn remap_multiboot(boot_info: &BootInformation, mapper: &mut PageMapper, allocator: &mut FrameLikeAllocator<Frame>) {
	let multiboot_start = Frame::from_address(PhysicalAddress::new_force_adjust(boot_info.start_address() as u64));
	let multiboot_end = Frame::from_address(PhysicalAddress::new_force_adjust(boot_info.end_address() as u64 - 1));
	for frame in FrameIter::inclusive(multiboot_start, multiboot_end) {
		let page = Page::from_address(VirtualAddress::new_adjusted(frame.start_address().raw() as usize));
		mapper.map_to(page, frame, EntryFlags::empty(), allocator);
	}
}

// Addresses from 0xF000_0000_0000 upwards are dedicated to the kernel,
// so we must ensure that the page directories for those addresses
// exist so we can clone the entire page table safely
fn prepare_kernel_page_directories(mapper: &mut PageMapper, allocator: &mut FrameLikeAllocator<Frame>) {
	for index in 480..510 {
		mapper.table_mut().create_if_nonexistent(index, allocator);
	}
}
