use memory::Frame;
use memory::HugeFrame;
use super::BootAllocator;
use super::FrameLikeAllocator;
use super::GenericAllocator;
use super::PostBootAllocator;

pub enum GlobalFrameAllocator {
	Boot(BootAllocator),
	PostBoot(PostBootAllocator),
}

impl GlobalFrameAllocator {
	fn allocator(&self) -> &GenericAllocator {
		use self::GlobalFrameAllocator::*;
		match self {
			Boot(allocator) => allocator,
			PostBoot(allocator) => allocator,
		}
	}

	fn allocator_mut(&mut self) -> &mut GenericAllocator {
		use self::GlobalFrameAllocator::*;
		match self {
			Boot(allocator) => allocator,
			PostBoot(allocator) => allocator,
		}
	}

	pub fn convert(&mut self, free_areas: ::alloc::Vec<::memory::MemoryArea>) {
		use core::mem::replace;
		use self::GlobalFrameAllocator::*;
		if let Boot(allocator) = self {
			replace(self, GlobalFrameAllocator::PostBoot(allocator.convert(free_areas)));
		} else {
			panic!("Cannot convert current allocator variant");
		}
	}
}

impl ::memory::FrameLikeAllocator<Frame> for GlobalFrameAllocator {
	fn free_frames_count(&self) -> usize {
		FrameLikeAllocator::<Frame>::free_frames_count(self.allocator())
	}

	fn used_frames_count(&self) -> usize {
		FrameLikeAllocator::<Frame>::used_frames_count(self.allocator())
	}

	fn allocate(&mut self) -> Option<Frame> {
		FrameLikeAllocator::<Frame>::allocate(self.allocator_mut())
	}

	fn deallocate(&mut self, frame: Frame) {
		FrameLikeAllocator::<Frame>::deallocate(self.allocator_mut(), frame);
	}
}

impl ::memory::FrameLikeAllocator<HugeFrame> for GlobalFrameAllocator {
	fn free_frames_count(&self) -> usize {
		FrameLikeAllocator::<HugeFrame>::free_frames_count(self.allocator())
	}

	fn used_frames_count(&self) -> usize {
		FrameLikeAllocator::<HugeFrame>::used_frames_count(self.allocator())
	}

	fn allocate(&mut self) -> Option<HugeFrame> {
		FrameLikeAllocator::<HugeFrame>::allocate(self.allocator_mut())
	}

	fn deallocate(&mut self, frame: HugeFrame) {
		FrameLikeAllocator::<HugeFrame>::deallocate(self.allocator_mut(), frame);
	}
}

impl super::GenericAllocator for GlobalFrameAllocator {}
