use memory::Frame;
use memory::frame_allocators::HugeFrameDivider;
use memory::huge_frame_allocators::huge_boot_bump_allocator::*;
use memory::HugeFrame;
use paging::Page;
use paging::PageLike;
use structures::FrameStore;
use super::FixedFrameRecycler;
use super::FrameLikeAllocator;
use super::FrameRecycler;
use super::PostBootAllocator;

pub struct BootAllocator {
	frame_allocator: Option<FixedFrameRecycler<Frame, HugeFrameDivider>>,
	huge_frame_allocator: HugeBootBumpAllocator,
}

// For information on the design of these allocators,
// see the mod.rs in memory/generic_allocators
impl BootAllocator {
	pub fn new(boot_structure: ::utility::MultibootStructure) -> BootAllocator {
		let mut huge_frame_allocator = HugeBootBumpAllocator::new(boot_structure);
		let initial_huge_frame = huge_frame_allocator.allocate()
		                                             .expect("Not enough memory");
		let frame_allocator = FixedFrameRecycler::new(HugeFrameDivider::new(initial_huge_frame));
		Self {
			frame_allocator: Some(frame_allocator),
			huge_frame_allocator,
		}
	}

	pub fn convert(&mut self, free_areas: ::alloc::Vec<::memory::MemoryArea>) -> PostBootAllocator {
		use memory::huge_frame_allocators::HugeBumpAllocator;
		let mut allocator_frame_store = self.allocator_frame_store();
		let huge_allocator_frame_store = self.huge_allocator_frame_store();

		let (mut free_frames, frame_allocator) = self.frame_allocator.take().unwrap().unwrap();
		for free_frame in &mut free_frames {
			if free_frame.is_some() {
				allocator_frame_store.push(free_frame.take().unwrap(), self);
			}
		}

		let frame_recycler = FrameRecycler::new_custom(frame_allocator, allocator_frame_store);

		let kernel_area = self.huge_frame_allocator.get_kernel_area();
		let next_frame = self.huge_frame_allocator.allocate().expect("Out of memory: Allocator conversion");
		let huge_bump_allocator = HugeBumpAllocator::new_custom(free_areas, kernel_area, next_frame);
		let huge_frame_recycler = FrameRecycler::new_custom(huge_bump_allocator, huge_allocator_frame_store);
		PostBootAllocator::new(frame_recycler, huge_frame_recycler)
	}

	fn allocator_frame_store(&mut self) -> FrameStore<Frame> {
		let start = Page::from_address(::paging::reserved::FRAME_STORE_BOTTOM);
		let end = Page::from_address(::paging::reserved::FRAME_STORE_TOP);
		let pages = ::paging::PageIter::inclusive(start, end);
		FrameStore::new(pages, self)
	}

	fn huge_allocator_frame_store(&mut self) -> FrameStore<HugeFrame> {
		use memory::FrameLike;

		let mut frame_store = {
			let start = Page::from_address(::paging::reserved::HUGE_FRAME_STORE_BOTTOM);
			let end = Page::from_address(::paging::reserved::HUGE_FRAME_STORE_TOP);
			let pages = ::paging::PageIter::inclusive(start, end);
			FrameStore::new(pages, self)
		};

		for used_area in self.huge_frame_allocator.used_areas.clone().into_iter().skip(1) {
			for free_area in self.huge_frame_allocator.free_areas.clone() {
				let free_area = ::memory::MemoryArea::from(free_area);
				if used_area.overlap(&free_area).is_some() {
					let start = HugeFrame::from_address(used_area.start_address());
					let end = HugeFrame::from_address(used_area.end_address());
					for frame in ::memory::FrameIter::inclusive(start, end) {
						frame_store.push(frame, self);
					}
				}
			}
		}

		frame_store
	}

	fn frame_allocator(&self) -> &FixedFrameRecycler<Frame, HugeFrameDivider> {
		self.frame_allocator.as_ref().unwrap()
	}

	fn frame_allocator_mut(&mut self) -> &mut FixedFrameRecycler<Frame, HugeFrameDivider> {
		self.frame_allocator.as_mut().unwrap()
	}
}

impl ::memory::FrameLikeAllocator<Frame> for BootAllocator {
	fn free_frames_count(&self) -> usize {
		(self.huge_frame_allocator.free_frames_count() * HugeFrameDivider::FRAME_COUNT) + self.frame_allocator().free_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		(self.huge_frame_allocator.used_frames_count() * HugeFrameDivider::FRAME_COUNT) - self.frame_allocator().free_frames_count()
	}

	fn allocate(&mut self) -> Option<Frame> {
		let frame = self.frame_allocator_mut().allocate();
		if frame.is_some() {
			return frame;
		}

		let huge_page = self.huge_frame_allocator.allocate()?;
		self.frame_allocator_mut().set(HugeFrameDivider::new(huge_page));
		self.frame_allocator_mut().allocate()
	}

	fn deallocate(&mut self, frame: Frame) {
		self.frame_allocator_mut().deallocate(frame);
	}
}

impl ::memory::FrameLikeAllocator<HugeFrame> for BootAllocator {
	fn free_frames_count(&self) -> usize {
		self.huge_frame_allocator.free_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		self.huge_frame_allocator.used_frames_count()
	}

	fn allocate(&mut self) -> Option<HugeFrame> {
		self.huge_frame_allocator.allocate()
	}

	fn deallocate(&mut self, _frame: HugeFrame) {
		panic!("BootAllocator does not support deallocation of huge frames")
	}
}

impl super::GenericAllocator for BootAllocator {}
