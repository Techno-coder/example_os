use alloc::boxed::Box;
use alloc::BTreeMap;
use graph::*;

pub struct Root {
	providers: BTreeMap<Identifier, Box<Provider + Send>>,
}

impl Root {
	pub fn new() -> Root {
		Root {
			providers: BTreeMap::new(),
		}
	}

	pub fn mount(&mut self, identifier: Identifier, provider: Box<Provider + Send>) {
		self.providers.insert(identifier, provider);
	}
}

impl Provider for Root {
	fn open(&mut self, location: &LocationSlice) -> Option<Box<Resource>> {
		let (first, rest) = location.split()?;
		self.providers.get_mut(first)?.open(&rest)
	}
}
