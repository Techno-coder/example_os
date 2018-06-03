use memory::Frame;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use memory::HugeFrame;
use super::EntryFlags;
use super::PageMapper;
use super::VirtualAddress;

pub trait PageLike: Clone {
	const SIZE: u64;
	type FrameType: FrameLike;

	fn from_index(index: usize) -> Self where Self: Sized;
	fn index(&self) -> usize;
	fn map_to(&self, table: &mut PageMapper, frame: Self::FrameType,
	          flags: EntryFlags, allocator: &mut FrameLikeAllocator<Frame>);
	fn un_map(&self, table: &mut PageMapper, allocator: &mut FrameLikeAllocator<Frame>) -> Self::FrameType;

	fn from_address(address: VirtualAddress) -> Self where Self: Sized {
		Self::from_index(address.raw() / Self::SIZE as usize)
	}

	fn start_address(&self) -> VirtualAddress where Self: Sized {
		VirtualAddress::new(self.index() * Self::SIZE as usize)
	}

	fn end_address(&self) -> VirtualAddress where Self: Sized {
		VirtualAddress::new((self.index() * Self::SIZE as usize) + (Self::SIZE as usize - 1))
	}
}

#[derive(Debug, Clone)]
pub struct Page {
	index: usize,
}

impl Page {
	pub fn table_4_index(&self) -> usize {
		(self.index >> 27) & 0o777
	}

	pub fn table_3_index(&self) -> usize {
		(self.index >> 18) & 0o777
	}

	pub fn table_2_index(&self) -> usize {
		(self.index >> 9) & 0o777
	}

	pub fn table_1_index(&self) -> usize {
		(self.index >> 0) & 0o777
	}
}

impl PageLike for Page {
	const SIZE: u64 = 4 * 1024;
	type FrameType = ::memory::Frame;

	fn from_index(index: usize) -> Self where Self: Sized {
		Self {
			index,
		}
	}

	fn index(&self) -> usize {
		self.index
	}

	fn map_to(&self, table: &mut PageMapper, frame: Self::FrameType,
	          flags: EntryFlags, allocator: &mut FrameLikeAllocator<Frame>) {
		let table_4 = table.table_mut();
		let table_3 = table_4.create_if_nonexistent(self.table_4_index(), allocator);
		let table_2 = table_3.create_if_nonexistent(self.table_3_index(), allocator);
		let table_1 = table_2.create_if_nonexistent(self.table_2_index(), allocator);

		assert!(table_1[self.table_1_index()].is_unused(), "Page at {:?} is already mapped", self.start_address());
		table_1[self.table_1_index()].set(frame, flags);
	}

	fn un_map(&self, table: &mut PageMapper, _allocator: &mut FrameLikeAllocator<Self::FrameType>) -> Frame {
		assert!(table.translate(&self.start_address()).is_some());

		let table_4 = table.table_mut();
		let table_1 = table_4.next_table_mut(self.table_4_index())
		                     .and_then(|table_3| table_3.next_table_mut(self.table_3_index()))
		                     .and_then(|table_2| table_2.next_table_mut(self.table_2_index()))
		                     .expect("Cannot remove mapping of huge page from normal page handler");
		let frame: Frame = table_1[self.table_1_index()].frame().unwrap();
		table_1[self.table_1_index()].set_unused();
		frame
	}
}

#[derive(Debug, Clone)]
pub struct HugePage {
	index: usize,
}

impl HugePage {
	fn table_4_index(&self) -> usize {
		(self.index >> 18) & 0o777
	}

	fn table_3_index(&self) -> usize {
		(self.index >> 9) & 0o777
	}

	fn table_2_index(&self) -> usize {
		(self.index >> 0) & 0o777
	}
}

impl PageLike for HugePage {
	const SIZE: u64 = 2 * 1024 * 1024;
	type FrameType = ::memory::HugeFrame;

	fn from_index(index: usize) -> Self where Self: Sized {
		Self {
			index,
		}
	}

	fn index(&self) -> usize {
		self.index
	}

	fn map_to(&self, table: &mut PageMapper, frame: Self::FrameType,
	          flags: EntryFlags, allocator: &mut FrameLikeAllocator<Frame>) {
		let table_4 = table.table_mut();
		let table_3 = table_4.create_if_nonexistent(self.table_4_index(), allocator);
		let table_2 = table_3.create_if_nonexistent(self.table_3_index(), allocator);

		assert!(table_2[self.table_2_index()].is_unused(), "Huge page at {:?} is already mapped", self.start_address());
		table_2[self.table_2_index()].set(frame, flags | EntryFlags::HUGE_PAGE);
	}

	fn un_map(&self, table: &mut PageMapper, _allocator: &mut FrameLikeAllocator<Frame>) -> HugeFrame {
		assert!(table.translate(&self.start_address()).is_some());

		let table_4 = table.table_mut();
		let table_2 = table_4.next_table_mut(self.table_4_index())
		                     .and_then(|table_3| table_3.next_table_mut(self.table_3_index()))
		                     .unwrap();
		let frame: HugeFrame = table_2[self.table_2_index()].frame().unwrap();
		table_2[self.table_2_index()].set_unused();
		frame
	}
}
