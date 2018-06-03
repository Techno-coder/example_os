use memory::Frame;
use super::FrameLikeAllocator;

macro_rules! frame_count { () => { 3 }; }

type FramePool = [Option<Frame>; frame_count!()];

pub struct TinyAllocator {
	pool: FramePool,
}

impl TinyAllocator {
	pub fn new(allocator: &mut FrameLikeAllocator<Frame>) -> TinyAllocator {
		let mut f = || allocator.allocate();
		let pool = [f(), f(), f()];
		TinyAllocator {
			pool,
		}
	}

	pub fn fill(&mut self, allocator: &mut FrameLikeAllocator<Frame>) {
		for frame in self.pool.iter_mut() {
			if frame.is_none() {
				*frame = Some(allocator.allocate().expect("Out of memory: TinyAllocator"));
			}
		}
	}

	pub fn dispose(&mut self, allocator: &mut FrameLikeAllocator<Frame>) {
		for frame in self.pool.iter_mut() {
			if let Some(frame) = frame.take() {
				allocator.deallocate(frame);
			}
		}
	}
}

impl FrameLikeAllocator<Frame> for TinyAllocator {
	fn free_frames_count(&self) -> usize {
		self.pool.iter().filter(|f| f.is_some()).count()
	}

	fn used_frames_count(&self) -> usize {
		self.pool.iter().filter(|f| f.is_none()).count()
	}

	fn allocate(&mut self) -> Option<Frame> {
		self.pool.iter_mut()
		    .find(|f| f.is_some())
		    .and_then(|f| f.take())
	}

	fn deallocate(&mut self, frame: Frame) {
		*self.pool.iter_mut()
		     .find(|f| f.is_none())
		     .expect(concat!("Tiny allocator can only hold ", frame_count!(), " frames"))
			= Some(frame);
	}
}
