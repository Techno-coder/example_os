use alloc::String;
use alloc::Vec;

pub struct Location {
	path: Vec<Identifier>,
}

impl Location {
	pub fn new(path: Vec<Identifier>) -> Location {
		Location {
			path,
		}
	}

	pub fn parse(string: &str) -> Location {
		let segments = string.split('/')
		                     .map(|string| Identifier::new(string))
		                     .collect();
		Self::new(segments)
	}

	pub fn as_slice(&self) -> LocationSlice {
		LocationSlice {
			path: &self.path,
		}
	}
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Identifier {
	identifier: String,
}

impl Identifier {
	pub fn new<S: ::alloc::string::ToString>(identifier: S) -> Identifier {
		Identifier {
			identifier: identifier.to_string(),
		}
	}
}

pub struct LocationSlice<'a> {
	path: &'a [Identifier],
}

impl<'a> LocationSlice<'a> {
	pub fn try_last(&self) -> Option<&'a Identifier> {
		if self.path.len() == 1 {
			self.path.last()
		} else {
			None
		}
	}

	pub fn split(&self) -> Option<(&'a Identifier, LocationSlice<'a>)> {
		let (first, rest) = self.path.split_first()?;
		Some((first, LocationSlice { path: rest }))
	}
}
