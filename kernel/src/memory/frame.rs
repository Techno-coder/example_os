use paging::PageLike;
use super::MemoryArea;
use super::PhysicalAddress;

pub trait FrameLike {
	type PageType: PageLike;

	fn size() -> u64 where Self: Sized;
	fn from_index(index: usize) -> Self where Self: Sized;
	fn index(&self) -> usize;

	fn from_address(address: PhysicalAddress) -> Self where Self: Sized {
		Self::from_index((address.raw() / Self::size()) as usize)
	}

	fn start_address(&self) -> PhysicalAddress where Self: Sized {
		PhysicalAddress::new(self.index() as u64 * Self::size())
	}

	fn end_address(&self) -> PhysicalAddress where Self: Sized {
		PhysicalAddress::new(self.start_address().raw() + (Self::size() - 1))
	}

	fn to_memory_area(&self) -> MemoryArea where Self: Sized {
		MemoryArea::new(self.start_address(), (Self::size() - 1) as usize)
	}
}

#[derive(Debug, Clone)]
pub struct FrameIter<F: FrameLike> {
	next: usize,
	end: usize,
	_frame_type: ::core::marker::PhantomData<F>,
}

impl<F: FrameLike> FrameIter<F> {
	pub fn inclusive(start: F, end: F) -> FrameIter<F> {
		assert!(start.index() <= end.index());
		Self::inclusive_unchecked(start, end)
	}

	pub fn inclusive_unchecked(start: F, end: F) -> FrameIter<F> {
		FrameIter {
			next: start.index(),
			end: end.index(),
			_frame_type: Default::default(),
		}
	}

	pub fn previous_next(&self) -> F {
		F::from_index(self.next - 1)
	}

	pub fn skip_to(&mut self, next: F) {
		self.next = next.index();
	}
}

impl<F: FrameLike> Iterator for FrameIter<F> {
	type Item = F;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		if self.next <= self.end {
			self.next += 1;
			Some(F::from_index(self.next - 1))
		} else {
			None
		}
	}
}

#[derive(Debug, Clone)]
pub struct Frame {
	index: usize,
}

impl Frame {
	pub const SIZE: u64 = ::paging::Page::SIZE;
}

impl FrameLike for Frame {
	type PageType = ::paging::Page;

	fn size() -> u64 {
		Frame::SIZE
	}
	fn from_index(index: usize) -> Frame {
		Frame {
			index,
		}
	}
	fn index(&self) -> usize {
		self.index
	}
}

impl PartialEq for Frame {
	fn eq(&self, other: &Self) -> bool {
		self.index().eq(&other.index())
	}
}

impl PartialOrd for Frame {
	fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
		self.index().partial_cmp(&other.index())
	}
}

#[derive(Debug, Clone)]
pub struct HugeFrame {
	index: usize,
}

impl HugeFrame {
	pub const SIZE: u64 = ::paging::HugePage::SIZE;
}

impl FrameLike for HugeFrame {
	type PageType = ::paging::HugePage;

	fn size() -> u64 {
		HugeFrame::SIZE
	}

	fn from_index(index: usize) -> HugeFrame {
		HugeFrame {
			index,
		}
	}

	fn index(&self) -> usize {
		self.index
	}
}

impl PartialEq for HugeFrame {
	fn eq(&self, other: &Self) -> bool {
		self.index().eq(&other.index())
	}
}

impl PartialOrd for HugeFrame {
	fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
		self.index().partial_cmp(&other.index())
	}
}
