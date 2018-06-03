use alloc::arc::Arc;
use alloc::Vec;
use graph::resource::*;
use spin::RwLock;

pub struct MemoryFile {
	data: Option<Arc<RwLock<Vec<u8>>>>,
	position: usize,
}

impl MemoryFile {
	pub fn new(data: Arc<RwLock<Vec<u8>>>) -> MemoryFile {
		MemoryFile {
			data: Some(data),
			position: 0,
		}
	}
}

impl Resource for MemoryFile {
	fn read(&mut self, buffer: &mut [u8]) -> ResourceResult<usize> {
		let data = self.data.as_ref().ok_or(ResourceError::Closed)?.read();
		let current_position = self.position;

		for byte in buffer {
			let data_byte = data.get(self.position);
			match data_byte {
				Some(data_byte) => {
					*byte = *data_byte;
					self.position += 1;
				}
				None => break,
			}
		}
		Ok(self.position - current_position)
	}

	fn write(&mut self, buffer: &[u8]) -> ResourceResult<()> {
		let mut data = self.data.as_mut().ok_or(ResourceError::Closed)?.write();
		for byte in buffer {
			if self.position == data.len() {
				data.push(*byte);
			} else {
				data[self.position] = *byte;
			}
			self.position += 1;
		}
		Ok(())
	}

	fn seek(&mut self, count: usize) -> ResourceResult<usize> {
		let data = self.data.as_ref().ok_or(ResourceError::Closed)?.read();
		let current_position = self.position;

		self.position += count;
		if self.position >= data.len() {
			self.position = data.len() - 1;
		}
		Ok(self.position - current_position)
	}

	fn close(&mut self) -> ResourceResult<()> {
		let _ = self.data.take();
		Ok(())
	}
}

impl Drop for MemoryFile {
	fn drop(&mut self) {
		let _ = self.close();
	}
}