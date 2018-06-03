pub type ResourceResult<T> = Result<T, ResourceError>;

pub trait Resource {
	fn read(&mut self, buffer: &mut [u8]) -> ResourceResult<usize>;
	fn write(&mut self, buffer: &[u8]) -> ResourceResult<()>;
	fn seek(&mut self, count: usize) -> ResourceResult<usize>;
	fn close(&mut self) -> ResourceResult<()>;
}

impl Resource {
	pub fn read_all(&mut self) -> ::alloc::Vec<u8> {
		let mut data = ::alloc::Vec::new();
		let mut buffer = [0];
		while let Ok(count) = self.read(&mut buffer) {
			if count == 0 { break; }
			data.push(buffer[0]);
		}
		data
	}
}

#[derive(Debug)]
pub enum ResourceError {
	Closed,
}
