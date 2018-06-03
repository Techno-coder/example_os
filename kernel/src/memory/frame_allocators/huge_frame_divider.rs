use memory::Frame;
use memory::FrameIter;
use memory::FrameLike;
use memory::HugeFrame;

#[derive(Debug, Clone)]
pub struct HugeFrameDivider {
	frames: FrameIter<Frame>,
	allocated_count: usize,
}

impl HugeFrameDivider {
	pub const FRAME_COUNT: usize = (HugeFrame::SIZE / Frame::SIZE) as usize;

	pub fn new(huge_frame: HugeFrame) -> HugeFrameDivider {
		let start_frame = Frame::from_address(huge_frame.start_address());
		let end_frame = Frame::from_address(huge_frame.end_address());
		HugeFrameDivider {
			frames: FrameIter::inclusive(start_frame, end_frame),
			allocated_count: 0,
		}
	}
}

impl super::FrameLikeAllocator<Frame> for HugeFrameDivider {
	fn free_frames_count(&self) -> usize {
		Self::FRAME_COUNT - self.used_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		self.allocated_count
	}

	fn allocate(&mut self) -> Option<Frame> {
		let frame = self.frames.next();
		if frame.is_some() {
			self.allocated_count += 1;
		}
		frame
	}

	fn deallocate(&mut self, _frame: Frame) {
		panic!("HugeFrameDivider does not support deallocation of frames")
	}
}
