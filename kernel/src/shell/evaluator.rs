use alloc::boxed::Box;
use alloc::BTreeMap;
use alloc::String;
use alloc::Vec;
use super::Process;

pub struct Evaluator {
	commands: BTreeMap<String, Traversal>,
}

impl Evaluator {
	pub fn new() -> Evaluator {
		Evaluator {
			commands: BTreeMap::new(),
		}
	}

	pub fn get_options(&self, prefix: &str) -> Vec<String> {
		self.commands.keys().filter_map(|string| {
			if string.starts_with(prefix) {
				return Some(string.clone());
			}
			None
		}).collect()
	}

	pub fn add_option(&mut self, command: &str, traversal: Traversal) {
		use alloc::string::ToString;
		self.commands.insert(command.to_string(), traversal);
	}

	pub fn traverse(&mut self, command: &String) -> Option<&mut Traversal> {
		self.commands.get_mut(command)
	}
}

pub enum Traversal {
	Process(Box<Process>),
	Evaluator(Evaluator),
}