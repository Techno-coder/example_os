use memory::FrameLike;
use structures::FrameStore;
use super::FrameLikeAllocator;

pub struct FrameRecycler<F, A> where F: FrameLike, A: FrameLikeAllocator<F> {
	allocator: A,
	free_frames: FrameStore<F>,
}

impl<F, A> FrameRecycler<F, A> where F: FrameLike, A: FrameLikeAllocator<F> {
	pub fn new_custom(allocator: A, free_frames: FrameStore<F>) -> FrameRecycler<F, A> {
		FrameRecycler {
			allocator,
			free_frames,
		}
	}

	pub fn set(&mut self, allocator: A) {
		self.allocator = allocator;
	}

	pub fn deallocate(&mut self, allocator: &mut FrameLikeAllocator<::memory::Frame>, frame: F) {
		self.free_frames.push(frame, allocator);
	}
}

impl<F, A> FrameLikeAllocator<F> for FrameRecycler<F, A>
	where F: FrameLike, A: FrameLikeAllocator<F> {
	fn free_frames_count(&self) -> usize {
		self.allocator.free_frames_count() + self.free_frames.size()
	}

	fn used_frames_count(&self) -> usize {
		self.allocator.used_frames_count() - self.free_frames.size()
	}

	fn allocate(&mut self) -> Option<F> {
		self.free_frames.pop().or_else(|| self.allocator.allocate())
	}

	fn deallocate(&mut self, _frame: F) {
		panic!("Deallocation requires a reference to a frame allocator")
	}
}
