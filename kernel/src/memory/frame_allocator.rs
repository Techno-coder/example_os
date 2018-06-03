use super::FrameLike;

pub trait FrameLikeAllocator<F: FrameLike> {
	fn free_frames_count(&self) -> usize;
	fn used_frames_count(&self) -> usize;

	fn allocate(&mut self) -> Option<F>;
	fn deallocate(&mut self, frame: F);
}
