pub trait Process: Send {
	fn run(&mut self);
}

pub struct ClosureProcess<F: FnMut()> {
	closure: F,
}

impl<F: FnMut()> ClosureProcess<F> where F: 'static + Send {
	pub fn new(closure: F) -> ClosureProcess<F> {
		ClosureProcess {
			closure,
		}
	}

	pub fn new_traversal(closure: F) -> super::Traversal {
		super::Traversal::Process(box Self::new(closure))
	}
}

impl<F: FnMut()> Process for ClosureProcess<F> where F: Send {
	fn run(&mut self) {
		(self.closure)();
	}
}