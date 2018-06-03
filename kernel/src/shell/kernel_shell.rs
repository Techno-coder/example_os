use alloc::String;
use alloc::Vec;
use super::Evaluator;
use super::ShellDisplay;
use super::Traversal;

pub struct KernelShell {
	display: ShellDisplay,
	initialized: bool,
	root: Evaluator,

	complete_options: Vec<String>,
	path: Vec<String>,
	control_key_held: bool,

	printed_count: usize,
}

impl KernelShell {
	const PROMPT: &'static str = "> ";

	pub fn new() -> KernelShell {
		KernelShell {
			display: ShellDisplay::new(),
			initialized: false,
			root: super::evaluators::root::construct(),
			complete_options: vec![],
			path: vec![],
			control_key_held: false,
			printed_count: 0,
		}
	}

	pub fn on_key_press(&mut self, key_event: ::keyboard::KeyEvent) {
		use keyboard::KeyState;
		use keyboard::KeyCode;
		use keyboard::key_other::KeySpecial;
		use keyboard::key_other::KeyModifier;

		let key_code = key_event.key_code;
		if key_event.state == KeyState::Released {
			if let KeyCode::Modifier(KeyModifier::ControlLeft) = key_code {
				self.control_key_held = false;
			}
			return;
		}

		self.try_initialize();
		match key_code {
			KeyCode::Modifier(KeyModifier::ControlLeft) => self.control_key_held = true,
			KeyCode::Special(KeySpecial::Backspace) => self.pop_printable(),
			KeyCode::Special(KeySpecial::Tab) => self.complete_option(),
			KeyCode::Special(KeySpecial::Enter) => {
				if self.run_process() {
					self.initialized = false;
				} else {
					self.reset_prompt();
					self.update_options();
				}
			}
			KeyCode::Character(character) => {
				let character = character.to_char();
				if !self.change_selection(character) {
					self.push_printable(character)
				}
			}
			KeyCode::Number(character) => self.push_printable(character.to_char()),
			KeyCode::Symbol(character) => self.push_printable(character.to_char()),
			_ => (),
		}
	}

	fn reset_prompt(&mut self) {
		self.display.print(Self::PROMPT);
		self.printed_count = Self::PROMPT.len();
		self.path = vec![String::new()];
	}

	fn change_selection(&mut self, character: char) -> bool {
		if !self.control_key_held { return false; }
		match character {
			'j' => self.display.offset_selected(1),
			'k' => self.display.offset_selected(-1),
			_ => ()
		}
		self.update_options();
		true
	}

	fn push_printable(&mut self, character: char) {
		if self.printed_count == self.display.main_width() { return; }
		self.printed_count += 1;

		self.display.push_character(character);
		if character == '.' {
			self.path.push(String::new());
		} else {
			self.path.last_mut().unwrap().push(character);
		}
		self.update_options();
	}

	fn pop_printable(&mut self) {
		if self.printed_count > Self::PROMPT.len() {
			self.printed_count -= 1;
		}

		if self.path.last().unwrap().len() == 0 {
			if self.path.len() > 1 {
				self.path.pop();
				self.display.pop_character();
			}
		} else {
			self.display.pop_character();
			self.path.last_mut().unwrap().pop();
		}
		self.update_options();
	}

	fn update_options(&mut self) {
		let options = self.get_options();
		self.display.set_options(&options);
		self.complete_options = options;
	}

	fn get_options(&mut self) -> Vec<String> {
		Self::traverse(&mut self.root, &self.path)
			.and_then(|(evaluator, last)| Some(evaluator.get_options(last)))
			.unwrap_or(vec![])
	}

	fn run_process(&mut self) -> bool {
		self.display.push_character('\n');

		if let Some((evaluator, last)) = Self::traverse(&mut self.root, &self.path) {
			if let Some(traversal) = evaluator.traverse(last) {
				return match traversal {
					Traversal::Evaluator(_) => {
						self.display.print_error("Shell path is a module not a process\n");
						false
					}
					Traversal::Process(process) => {
						process.run();
						true
					}
				};
			}
		}

		self.display.print_error("Unknown shell path to process\n");
		false
	}

	fn traverse<'a>(current: &'a mut Evaluator, rest: &'a [String]) -> Option<(&'a mut Evaluator, &'a String)> {
		if rest.len() == 1 {
			return Some((current, &rest[0]));
		}

		let (next, rest) = rest.split_first()?;
		match current.traverse(&next)? {
			Traversal::Evaluator(ref mut evaluator) => Self::traverse(evaluator, rest),
			_ => None,
		}
	}

	fn complete_option(&mut self) {
		if self.complete_options.is_empty() { return; }

		let selected = self.display.get_selected();
		{
			let option = &self.complete_options[selected];
			let remaining = &option[self.path.last().unwrap().len()..];

			if self.printed_count + remaining.len() > self.display.main_width() { return; }
			self.printed_count += remaining.len();

			self.display.print(remaining);
			self.path.last_mut().unwrap().push_str(remaining);
		}

		self.update_options();
	}

	fn try_initialize(&mut self) {
		if !self.initialized {
			self.display.redraw();
			self.reset_prompt();
			self.update_options();
			self.initialized = true;
		}
	}
}