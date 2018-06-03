use super::PageLike;

#[derive(Debug, Clone)]
pub struct PageIter<P: PageLike> {
	start: usize,
	next: usize,
	end: usize,
	_page_type: ::core::marker::PhantomData<P>,
}

impl<P: PageLike> PageIter<P> {
	pub fn inclusive(start: P, end: P) -> PageIter<P> {
		assert!(start.index() <= end.index());
		PageIter {
			start: start.index(),
			next: start.index(),
			end: end.index(),
			_page_type: Default::default(),
		}
	}
}

impl<P: PageLike> Iterator for PageIter<P> {
	type Item = P;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		if self.next <= self.end {
			self.next += 1;
			Some(P::from_index(self.next - 1))
		} else {
			None
		}
	}
}
