use alloc::boxed::Box;
use super::LocationSlice;
use super::Resource;

pub trait Provider {
	fn open(&mut self, location: &LocationSlice) -> Option<Box<Resource>>;
}
