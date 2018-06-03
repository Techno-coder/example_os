use paging::Page;
use paging::PageLike;

type StackData = [u8; Page::SIZE as usize];

/// An uninitialized stack the size of one normal page
pub struct FixedStack {
	stack: StackData,
}

impl FixedStack {
	pub const fn new() -> FixedStack {
		FixedStack {
			stack: [0; Page::SIZE as usize],
		}
	}

	pub fn address(&mut self) -> usize {
		(&mut self.stack[Page::SIZE as usize - 1] as *mut _) as _
	}
}
