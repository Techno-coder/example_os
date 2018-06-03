use alloc::Vec;
use core::ops::DerefMut;
use memory::Frame;
use memory::FRAME_ALLOCATOR;
use memory::FrameLikeAllocator;
use paging::Page;
use paging::PageLike;

/// Returns false if the given frame referenced by the page is erroneous
fn is_good_frame(page: Page) -> bool {
	use utility::PseudoRandomGenerator;
	const MAX_INDEX: usize = (Page::SIZE as usize / 8) - 1;
	let seeded_generator = || PseudoRandomGenerator::new(page.start_address().raw() as u64);

	let mut generator = seeded_generator();
	for index in 0..MAX_INDEX {
		let mut region = (page.start_address().raw() + index * 8) as *mut u64;
		unsafe { *region = generator.next() };
	}

	let mut generator = seeded_generator();
	for index in 0..MAX_INDEX {
		let mut region = (page.start_address().raw() + index * 8) as *mut u64;
		unsafe {
			if *region != generator.next() {
				return false;
			}
		}
	}
	return true;
}

fn allocate_frames(free_frames_count: usize) -> (u64, usize, Vec<Option<Frame>>) {
	use paging::ACTIVE_PAGE_TABLE;
	use paging::EntryFlags;
	let mut frames: Vec<Option<Frame>> = vec![None; free_frames_count];

	let mut frame_allocator = FRAME_ALLOCATOR.lock();
	let frame_allocator = frame_allocator.deref_mut();
	let mut page_table = ACTIVE_PAGE_TABLE.lock();
	let page_table = page_table.deref_mut();

	let mut allocation_count = 0;
	let mut erroneous_frames_count = 0;
	let temporary_page = Page::from_address(::paging::reserved::TEMPORARY_PAGE);
	loop {
		let frame: Frame;
		if let Some(new_frame) = frame_allocator.allocate() {
			frame = new_frame;
		} else {
			break;
		}

		page_table.map_to(temporary_page.clone(), frame, EntryFlags::WRITABLE, frame_allocator);
		if is_good_frame(temporary_page.clone()) {
			print!(".");
		} else {
			erroneous_frames_count += 1;
			eprint!("x");
		}
		let frame = page_table.un_map(temporary_page.clone(), frame_allocator);

		frames[allocation_count] = Some(frame);
		allocation_count += 1;
		if allocation_count % 64 == 0 {
			println!(" {} frames", allocation_count);
		}
	}

	(erroneous_frames_count, allocation_count, frames)
}

fn deallocate_frames(mut frames: Vec<Option<Frame>>) {
	for allocated_frame in frames.iter_mut() {
		let frame: Frame;
		if let Some(new_frame) = allocated_frame.take() {
			frame = new_frame;
		} else {
			return;
		}

		FRAME_ALLOCATOR.lock().deallocate(frame);
	}
}

fn memory_test() {
	use utility::math::percentage;
	use core::ops::Deref;
	use display::text_mode::LowDepthColour;
	use display::text_mode::Printer;

	let free_frames_count = FrameLikeAllocator::<Frame>::free_frames_count(FRAME_ALLOCATOR.lock().deref());
	let (erroneous_frames_count, allocation_count, frames) = allocate_frames(free_frames_count);
	deallocate_frames(frames);
	let bad_frames_percentage = percentage(erroneous_frames_count, allocation_count as u64);

	println!("\n");
	println!("==================================");
	println!("Projected free frames count: {}", free_frames_count);
	println!("Actual checked frames: {}", allocation_count);
	if erroneous_frames_count > 0 {
		eprintln!("Erroneous frames: {} ({}%)", erroneous_frames_count, bad_frames_percentage);
	} else {
		let mut printer = ::display::text_mode::SYSTEM_PRINTER.lock();
		printer.print_coloured("Erroneous frames: 0 (0%)\n", &LowDepthColour::BACKGROUND, &LowDepthColour::LightGreen);
	}
	println!("==================================");
}

pub fn memory_test_process() -> super::Traversal {
	use shell::ClosureProcess;
	ClosureProcess::new_traversal(|| memory_test())
}
