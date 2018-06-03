use core::ops::Deref;
use memory::*;
use shell::ClosureProcess;
use super::Evaluator;
use super::Traversal;

pub fn construct() -> Evaluator {
	let mut evaluator = Evaluator::new();
	evaluator.add_option("available", available());
	evaluator.add_option("test", super::memory_test::memory_test_process());
	evaluator
}

fn available() -> Traversal {
	ClosureProcess::new_traversal(|| {
		let free_frames = FrameLikeAllocator::<Frame>::free_frames_count(FRAME_ALLOCATOR.lock().deref());
		let used_frames = FrameLikeAllocator::<Frame>::used_frames_count(FRAME_ALLOCATOR.lock().deref());
		let total_frames = free_frames + used_frames;
		println!("{} megabytes total memory ({} frames)", total_frames / 256, total_frames);
		println!("{} megabytes of free memory ({} frames)", free_frames / 256, free_frames);
		println!("{} megabytes of used memory ({} frames)", used_frames / 256, used_frames);
	})
}

