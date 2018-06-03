use core::ptr::NonNull;
use memory::Frame;
use memory::FrameLike;
use memory::FrameLikeAllocator;
use paging::EntryFlags;
use paging::Page;
use paging::PageIter;
use paging::PageLike;

pub const MAX_FRAMES: usize = 255;

// This structure has been created for the
// express purpose of not using heap allocation
// to store the frames. Using it causes a deadlock
// due to the way heap allocation works in
// the kernel. See memory/functions::handle_heap_fault
pub struct FrameStore<F: FrameLike> {
	pages: PageIter<Page>,
	top: NonNull<FrameStoreNode<F>>,
	size: usize,
}

impl<F: FrameLike> FrameStore<F> {
	pub fn new(mut pages: PageIter<Page>, allocator: &mut FrameLikeAllocator<Frame>) -> FrameStore<F> {
		let root = Self::construct_node(pages.next().expect("Out of pages"), allocator);
		FrameStore {
			pages,
			top: root,
			size: 0,
		}
	}

	pub fn push(&mut self, frame: F, allocator: &mut FrameLikeAllocator<Frame>) {
		let index = self.size % MAX_FRAMES;
		if index == 0 && self.size != 0 {
			if unsafe { self.top.as_mut() }.next.is_none() {
				let page = self.pages.next().expect("Out of pages");
				let mut node = Self::construct_node(page, allocator);
				unsafe {
					node.as_mut().previous = Some(self.top);
					self.top.as_mut().next = Some(node);
				}
			}
			self.top = unsafe { self.top.as_mut() }.next.unwrap();
		}
		unsafe { self.top.as_mut() }.frames[index] = Some(frame);
		self.size += 1;
	}

	pub fn pop(&mut self) -> Option<F> {
		if self.size == 0 {
			return None;
		}

		self.size -= 1;
		let index = self.size % MAX_FRAMES;
		let frame = unsafe { self.top.as_mut() }.frames[index].take().unwrap();

		if self.size != 0 && index == 0 {
			self.top = unsafe { self.top.as_mut() }.previous.unwrap();
		}
		Some(frame)
	}

	pub fn size(&self) -> usize {
		self.size
	}

	fn construct_node(page: Page, allocator: &mut FrameLikeAllocator<Frame>) -> NonNull<FrameStoreNode<F>> {
		let frame = allocator.allocate().expect("Out of memory: FrameStore");
		let mut table = ::paging::ACTIVE_PAGE_TABLE.lock();
		table.map_to(page.clone(), frame, EntryFlags::WRITABLE, allocator);

		let node = page.start_address().raw() as *mut FrameStoreNode<F>;
		let node = NonNull::new(node).unwrap();
		unsafe { ::core::ptr::write(node.as_ptr(), FrameStoreNode::new()); }
		node
	}
}

unsafe impl<F: FrameLike> Send for FrameStore<F> {}

pub struct FrameStoreNode<F: FrameLike> {
	pub previous: Option<NonNull<FrameStoreNode<F>>>,
	pub next: Option<NonNull<FrameStoreNode<F>>>,
	pub frames: [Option<F>; MAX_FRAMES],
}

impl<F: FrameLike> FrameStoreNode<F> {
	pub fn new() -> FrameStoreNode<F> {
		FrameStoreNode {
			previous: None,
			next: None,
			frames: unsafe { ::core::mem::uninitialized() },
		}
	}
}
