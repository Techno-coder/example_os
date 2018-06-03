use memory::Frame;
use memory::frame_allocators::HugeFrameDivider;
use memory::frame_allocators::TinyAllocator;
use memory::huge_frame_allocators::HugeBumpAllocator;
use memory::HugeFrame;
use super::FrameLikeAllocator;
use super::FrameRecycler;

pub struct PostBootAllocator {
	frame_allocator: FrameRecycler<Frame, HugeFrameDivider>,
	huge_frame_allocator: FrameRecycler<HugeFrame, HugeBumpAllocator>,
	pool: Option<TinyAllocator>,
}

impl PostBootAllocator {
	pub fn new(mut frame_allocator: FrameRecycler<Frame, HugeFrameDivider>,
	           huge_frame_allocator: FrameRecycler<HugeFrame, HugeBumpAllocator>) -> PostBootAllocator {
		let pool = TinyAllocator::new(&mut frame_allocator);
		Self {
			frame_allocator,
			huge_frame_allocator,
			pool: Some(pool),
		}
	}

	fn take_pool(&mut self) -> TinyAllocator {
		let mut pool = self.pool.take().unwrap();
		pool.fill(self);
		pool
	}
}

impl FrameLikeAllocator<Frame> for PostBootAllocator {
	fn free_frames_count(&self) -> usize {
		(self.huge_frame_allocator.free_frames_count() * HugeFrameDivider::FRAME_COUNT)
			+ self.frame_allocator.free_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		(self.huge_frame_allocator.used_frames_count() * HugeFrameDivider::FRAME_COUNT)
			- self.frame_allocator.free_frames_count()
	}

	fn allocate(&mut self) -> Option<Frame> {
		let frame = self.frame_allocator.allocate();
		if frame.is_some() {
			return frame;
		}

		let huge_page = self.huge_frame_allocator.allocate()?;
		self.frame_allocator.set(HugeFrameDivider::new(huge_page));
		self.frame_allocator.allocate()
	}

	fn deallocate(&mut self, frame: Frame) {
		let mut pool = self.take_pool();
		self.frame_allocator.deallocate(&mut pool, frame);
		self.pool = Some(pool);
	}
}

impl FrameLikeAllocator<HugeFrame> for PostBootAllocator {
	fn free_frames_count(&self) -> usize {
		self.huge_frame_allocator.free_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		self.huge_frame_allocator.used_frames_count()
	}

	fn allocate(&mut self) -> Option<HugeFrame> {
		self.huge_frame_allocator.allocate()
	}

	fn deallocate(&mut self, frame: HugeFrame) {
		let mut pool = self.take_pool();
		self.huge_frame_allocator.deallocate(&mut pool, frame);
		self.pool = Some(pool);
	}
}

impl super::GenericAllocator for PostBootAllocator {}
