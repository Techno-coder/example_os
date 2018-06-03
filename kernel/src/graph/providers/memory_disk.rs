use alloc::arc::Arc;
use alloc::boxed::Box;
use alloc::BTreeMap;
use alloc::String;
use alloc::string::ToString;
use alloc::Vec;
use graph::*;
use graph::resources::MemoryFile;
use spin::RwLock;

pub struct MemoryDisk {
	files: BTreeMap<Identifier, Arc<RwLock<Vec<u8>>>>,
	folders: BTreeMap<Identifier, MemoryDisk>,
}

impl MemoryDisk {
	const SECTOR_SIZE: usize = 512;

	pub fn new() -> MemoryDisk {
		MemoryDisk {
			files: BTreeMap::new(),
			folders: BTreeMap::new(),
		}
	}

	pub fn parse_archive(archive_data: &[u8]) -> Option<MemoryDisk> {
		let mut memory_disk = Self::new();

		let mut cursor = 0;
		while cursor <= (archive_data.len() - Self::SECTOR_SIZE) {
			let (file_path, data, new_cursor) = match Self::parse_file(cursor, archive_data) {
				Some(data) => data,
				None => break,
			};

			let location = Location::parse(&file_path[2..]);
			Self::add_file(&mut memory_disk, &location.as_slice(), data);

			cursor = ::utility::math::align_up_usize(new_cursor, Self::SECTOR_SIZE);
		}

		Some(memory_disk)
	}

	fn add_file(current: &mut MemoryDisk, path: &LocationSlice, data: Vec<u8>) {
		if let Some(last) = path.try_last() {
			let data = Arc::new(RwLock::new(data));
			current.files.insert(last.clone(), data);
			return;
		}

		let (first, rest) = path.split().unwrap();
		let next = match current.folders.get_mut(first) {
			Some(next) => return Self::add_file(next, &rest, data),
			None => {
				let mut next = MemoryDisk::new();
				Self::add_file(&mut next, &rest, data);
				next
			}
		};
		current.folders.insert(first.clone(), next);
	}

	fn parse_file(cursor: usize, archive_data: &[u8]) -> Option<(String, Vec<u8>, usize)> {
		if &archive_data[cursor + 257..cursor + 257 + 5] != b"ustar" { return None; }
		let file_path = Self::parse_file_path(cursor, archive_data)?;
		let file_size = Self::parse_file_size(cursor, archive_data)?;

		let file_data_start = cursor + Self::SECTOR_SIZE;
		let file_data_end = file_data_start + file_size;
		let file_data = archive_data[file_data_start..file_data_end].to_vec();
		Some((file_path, file_data, file_data_end))
	}

	fn parse_file_path(cursor: usize, archive_data: &[u8]) -> Option<String> {
		let file_path = archive_data[cursor..cursor + 100].to_vec();
		let file_path = String::from_utf8(file_path).ok()?;
		Some(file_path.trim_matches(0 as char).to_string())
	}

	fn parse_file_size(cursor: usize, archive_data: &[u8]) -> Option<usize> {
		let file_size_octal = archive_data[cursor + 0x7c..cursor + 0x7c + 11].to_vec();
		let file_size_octal = String::from_utf8(file_size_octal).ok()?;
		usize::from_str_radix(&file_size_octal, 8).ok()
	}
}

impl Provider for MemoryDisk {
	fn open(&mut self, location: &LocationSlice) -> Option<Box<Resource>> {
		if let Some(last) = location.try_last() {
			let resource = self.files.get(last)?.clone();
			return Some(box MemoryFile::new(resource));
		}

		let (first, rest) = location.split()?;
		self.folders.get_mut(first)?.open(&rest)
	}
}
