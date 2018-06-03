use memory::FrameIter;
use memory::FrameLike;
use memory::HugeFrame;
use memory::MemoryArea;
use memory::PhysicalAddress;
use super::FrameLikeAllocator;
use utility::MultibootStructure;

type UsedAreas = [MemoryArea; 2];

pub struct HugeBootBumpAllocator {
	boot_structure: MultibootStructure,
	next_huge_frame: FrameIter<HugeFrame>,

	pub used_areas: UsedAreas,
	pub free_areas: ::multiboot2::MemoryAreaIter,

	total_frames: usize,
	used_frames: usize,
}

impl HugeBootBumpAllocator {
	pub fn new(boot_structure: MultibootStructure) -> HugeBootBumpAllocator {
		let boot_information = boot_structure.get();
		let free_areas = boot_information.memory_map_tag()
		                                 .expect("MemoryMapTag not available")
		                                 .memory_areas();
		let used_areas = Self::create_used_areas(&boot_information);
		let (total_frames, used_frames) = Self::calculate_frames(&boot_information, &used_areas);
		let dummy_frame = HugeFrame::from_index(0);
		let mut allocator = HugeBootBumpAllocator {
			boot_structure,
			next_huge_frame: FrameIter::inclusive(dummy_frame.clone(), dummy_frame),
			used_areas,
			free_areas,
			total_frames,
			used_frames,
		};
		allocator.next_huge_frame = allocator.next_free_area().expect("Not enough memory");
		allocator
	}

	pub fn get_kernel_area(&self) -> MemoryArea {
		Self::create_kernel_area(&self.boot_structure.get().elf_sections_tag().unwrap())
	}

	fn create_kernel_area(elf_sections: &::multiboot2::ElfSectionsTag) -> MemoryArea {
		let kernel_start = elf_sections.sections().map(|s| s.start_address()).min().unwrap();
		let kernel_end = elf_sections.sections().map(|s| s.end_address()).max().unwrap() - ::KERNEL_BASE as u64;
		MemoryArea::new(PhysicalAddress::new(kernel_start), (kernel_end - kernel_start) as usize)
	}

	fn create_used_areas(boot_information: &::multiboot2::BootInformation) -> UsedAreas {
		let elf_sections = boot_information.elf_sections_tag().expect("ElfSectionsTag not available");

		let multiboot_size = boot_information.total_size();
		let multiboot_start = boot_information.start_address() as u64;

		[
			Self::create_kernel_area(&elf_sections),
			MemoryArea::new(PhysicalAddress::new_force_adjust(multiboot_start), multiboot_size),
		]
	}

	fn next_free_area(&mut self) -> Option<FrameIter<HugeFrame>> {
		let next_free_area: MemoryArea = self.free_areas.next()?.into();

		let start = next_free_area.start_address().align_up(HugeFrame::SIZE);
		let end = next_free_area.end_address().align_down(HugeFrame::SIZE).raw();
		let end = PhysicalAddress::new(end.saturating_sub(1));
		if (end.raw() - start.raw()) < HugeFrame::SIZE {
			return self.next_free_area();
		}

		Some(FrameIter::inclusive(HugeFrame::from_address(start),
		                          HugeFrame::from_address(end)))
	}

	/// Returns true if the frame does not overlap any used areas
	fn validate_frame(&mut self, frame: &HugeFrame) -> bool {
		let validate_area = |used_area: &MemoryArea| -> Option<HugeFrame> {
			if used_area.overlap(&frame.to_memory_area()).is_some() {
				let next_valid_frame_address = used_area.end_address().align_up(HugeFrame::SIZE);
				return Some(HugeFrame::from_address(next_valid_frame_address));
			}
			None
		};

		for used_area in &self.used_areas {
			if let Some(next_valid_frame) = validate_area(used_area) {
				self.next_huge_frame.skip_to(next_valid_frame);
				return false;
			}
		}

		for module_area in self.module_areas() {
			if let Some(next_valid_frame) = validate_area(&module_area.into()) {
				self.next_huge_frame.skip_to(next_valid_frame);
				return false;
			}
		}

		true
	}

	/// Returns the initial total and used frames
	fn calculate_frames(boot_information: &::multiboot2::BootInformation, used_areas: &UsedAreas) -> (usize, usize) {
		let free_areas = boot_information.memory_map_tag().unwrap().memory_areas();
		let mut total_frames = 0;
		let mut used_frames = 0;

		{
			let mut add_overlap = |used_area: &MemoryArea| {
				let start = HugeFrame::from_address(used_area.start_address());
				let end = HugeFrame::from_address(used_area.end_address());
				used_frames += end.index() - start.index() + 1;
			};

			for used_area in used_areas.iter() {
				add_overlap(used_area);
			}

			for module in boot_information.module_tags() {
				add_overlap(&module.into())
			}
		}

		for free_area in free_areas {
			let free_area: MemoryArea = free_area.into();
			let start = free_area.start_address().align_up(HugeFrame::SIZE);
			let end = free_area.end_address().align_down(HugeFrame::SIZE).raw();
			let end = PhysicalAddress::new(end.saturating_sub(1));
			total_frames += (HugeFrame::from_address(end).index()
				- HugeFrame::from_address(start).index()) + 1;
		}
		(total_frames, used_frames)
	}

	fn module_areas(&self) -> ::multiboot2::ModuleIter {
		self.boot_structure.get().module_tags()
	}
}

impl FrameLikeAllocator<HugeFrame> for HugeBootBumpAllocator {
	/// Returns an estimate of the amount of frames left for allocation
	///
	/// Count is understated due to two or more distinct used areas
	/// being in the same huge frame
	///
	fn free_frames_count(&self) -> usize {
		self.total_frames.saturating_sub(self.used_frames_count())
	}

	fn used_frames_count(&self) -> usize {
		self.used_frames
	}

	fn allocate(&mut self) -> Option<HugeFrame> {
		let frame = self.next_huge_frame.next();
		match frame {
			Some(frame) => {
				if self.validate_frame(&frame) {
					self.used_frames += 1;
					return Some(frame);
				}
			}
			None => {
				self.next_huge_frame = self.next_free_area()?;
			}
		}
		self.allocate()
	}

	fn deallocate(&mut self, _frame: HugeFrame) {
		panic!("HugeBootBumpAllocator does not support deallocation of huge frames")
	}
}
