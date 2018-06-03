use alloc::Vec;
use memory::FrameIter;
use memory::FrameLike;
use memory::HugeFrame;
use memory::MemoryArea;
use memory::PhysicalAddress;

pub struct HugeBumpAllocator {
	free_areas: Vec<MemoryArea>,
	next_free_area: usize,

	next_huge_frame: FrameIter<HugeFrame>,
	kernel: MemoryArea,

	total_frames: usize,
	used_frames: usize,
}

impl HugeBumpAllocator {
	pub fn new_custom(free_areas: Vec<MemoryArea>, kernel: MemoryArea,
	                  next_frame: HugeFrame) -> Self {
		let (total_frames, used_frames) = Self::calculate_frames(&free_areas, &kernel, &next_frame);
		HugeBumpAllocator {
			free_areas,
			next_free_area: 0,
			next_huge_frame: FrameIter::inclusive(next_frame.clone(), next_frame),
			kernel,
			total_frames,
			used_frames,
		}
	}

	fn calculate_frames(free_areas: &[MemoryArea], kernel: &MemoryArea, next_frame: &HugeFrame) -> (usize, usize) {
		let mut total_frames = 0;
		let mut used_frames = 0;

		for area in free_areas {
			let start = area.start_address().align_up(HugeFrame::SIZE);
			let end = area.end_address().align_down(HugeFrame::SIZE).raw();
			let end = PhysicalAddress::new(end.saturating_sub(1));
			let end_frame = HugeFrame::from_address(end);
			let start_frame = HugeFrame::from_address(start);
			total_frames += (end_frame.index() - start_frame.index()) + 1;

			if &start_frame <= next_frame && next_frame <= &end_frame {
				used_frames += (next_frame.index() - start_frame.index()) + 1;
			}
		}

		if kernel.start_address() > next_frame.to_memory_area().end_address() {
			let start = kernel.start_address().align_down(HugeFrame::SIZE);
			let end = kernel.end_address().align_up(HugeFrame::SIZE).raw();
			let end = PhysicalAddress::new(end.saturating_sub(1));
			let end_frame = HugeFrame::from_address(end);
			let start_frame = HugeFrame::from_address(start);
			used_frames += (end_frame.index() - start_frame.index()) + 1;
		}

		(total_frames, used_frames)
	}

	fn select_next_free_area(&mut self) -> Option<FrameIter<HugeFrame>> {
		let mut start: PhysicalAddress;
		let mut end: PhysicalAddress;

		{
			let next_free_area = self.free_areas.get(self.next_free_area)?;
			self.next_free_area += 1;

			start = next_free_area.start_address().align_up(HugeFrame::SIZE);
			let end_raw = next_free_area.end_address().align_down(HugeFrame::SIZE).raw();
			end = PhysicalAddress::new(end_raw.saturating_sub(1));
		}

		if self.next_huge_frame.previous_next().end_address() > start {
			start = self.next_huge_frame.previous_next().end_address().align_up(HugeFrame::SIZE);
		}

		if self.kernel.start_address() <= start && start <= self.kernel.end_address() {
			start = self.kernel.end_address().align_up(HugeFrame::SIZE);
		}

		if self.kernel.start_address() <= end && end <= self.kernel.end_address() {
			let end_raw = self.kernel.start_address().align_down(HugeFrame::SIZE).raw();
			end = PhysicalAddress::new(end_raw.saturating_sub(1));
		}

		if (end.raw().saturating_sub(start.raw())) < HugeFrame::SIZE {
			return self.select_next_free_area();
		}

		Some(FrameIter::inclusive(HugeFrame::from_address(start),
		                          HugeFrame::from_address(end)))
	}
}

impl super::FrameLikeAllocator<HugeFrame> for HugeBumpAllocator {
	fn free_frames_count(&self) -> usize {
		self.total_frames - self.used_frames_count()
	}

	fn used_frames_count(&self) -> usize {
		self.used_frames
	}

	fn allocate(&mut self) -> Option<HugeFrame> {
		let frame = self.next_huge_frame.next();
		if frame.is_none() {
			self.next_huge_frame = self.select_next_free_area()?;
			return self.allocate();
		}
		self.used_frames += 1;
		frame
	}

	fn deallocate(&mut self, _frame: HugeFrame) {
		panic!("HugeBumpAllocator does not support deallocation of huge frames")
	}
}
