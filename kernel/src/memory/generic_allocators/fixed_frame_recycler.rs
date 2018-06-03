use memory::FrameLike;
use super::FrameLikeAllocator;

macro_rules! amount {
    () => { 16 };
}

pub const MAX_FRAMES: usize = amount!();

type Frames<F> = [Option<F>; MAX_FRAMES];

pub struct FixedFrameRecycler<F, A> where F: FrameLike, A: FrameLikeAllocator<F> {
	allocator: A,
	free_frames: Frames<F>,
	used_frames: usize,
}

impl<F, A> FixedFrameRecycler<F, A> where F: FrameLike, A: FrameLikeAllocator<F> {
	pub fn new(allocator: A) -> FixedFrameRecycler<F, A> {
		Self {
			allocator,
			free_frames: Default::default(),
			used_frames: 0,
		}
	}

	pub fn set(&mut self, allocator: A) {
		self.allocator = allocator;
	}

	pub fn unwrap(self) -> (Frames<F>, A) {
		(self.free_frames, self.allocator)
	}

	fn free_frames(&self) -> usize {
		MAX_FRAMES - self.used_frames
	}
}

impl<F, A> FrameLikeAllocator<F> for FixedFrameRecycler<F, A>
	where F: FrameLike, A: FrameLikeAllocator<F> {
	fn free_frames_count(&self) -> usize {
		self.allocator.free_frames_count() + self.free_frames()
	}

	fn used_frames_count(&self) -> usize {
		self.allocator.used_frames_count() - self.free_frames()
	}

	fn allocate(&mut self) -> Option<F> {
		self.free_frames.iter_mut()
		    .find(|f| f.is_some())
		    .and_then(|f| f.take())
		    .or_else(|| self.allocator.allocate())
	}

	fn deallocate(&mut self, frame: F) {
		*self.free_frames.iter_mut()
		     .find(|f| f.is_none())
		     .expect(concat!("FixedFrameRecycler can only hold ", amount!(), " free frames")) = Some(frame);
	}
}
