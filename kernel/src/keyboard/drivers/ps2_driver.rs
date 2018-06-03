use keyboard::key_event::*;
use keyboard::key_other::*;
use keyboard::key_printable::*;

pub struct PS2Driver {
	shift_left_pressed: KeyState,
	shift_right_pressed: KeyState,
	caps_lock_on: bool,
}

impl PS2Driver {
	pub const fn new() -> PS2Driver {
		PS2Driver {
			shift_left_pressed: KeyState::Released,
			shift_right_pressed: KeyState::Released,
			caps_lock_on: false,
		}
	}

	pub fn parse_port_input(&mut self) -> Option<KeyEvent> {
		let mut scan_code = Self::read();
		let mut auxiliary_code = 0x0;
		let mut state = KeyState::Pressed;

		match scan_code {
			0xe0 => {
				auxiliary_code = Self::read();
				if let Some(event) = PS2Driver::match_print_code(scan_code, auxiliary_code) {
					Self::discard(2);
					return Some(event);
				}

				if auxiliary_code >= 0x90 {
					auxiliary_code -= 0x80;
					state = KeyState::Released;
				}
			}
			0xe1 => {
				Self::discard(5);
				return Some(KeyEvent {
					key_code: KeySpecial::Pause.into(),
					state: KeyState::Pressed,
				});
			}
			_ => {
				if scan_code >= 0x81 {
					scan_code -= 0x80;
					state = KeyState::Released;
				}
			}
		}

		let key_letter = self.parse_key_letter(scan_code, state);
		if key_letter.is_some() {
			return key_letter;
		}

		let key_code = self.parse_key_code(scan_code, auxiliary_code)?;
		self.update_state(&key_code, state);

		Some(KeyEvent {
			key_code,
			state,
		})
	}

	fn parse_key_letter(&self, scan_code: u8, state: KeyState) -> Option<KeyEvent> {
		let letter = PS2Driver::match_letter_code(scan_code)?;
		Some(KeyEvent {
			key_code: KeyCharacter {
				letter,
				is_capital: self.shift_pressed() || self.caps_lock_on,
			}.into(),
			state,
		})
	}

	fn parse_key_code(&self, scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		PS2Driver::match_key_pad_code(scan_code, auxiliary_code)
			.or_else(|| {
				if self.shift_pressed() {
					PS2Driver::match_number_symbols(scan_code)
				} else {
					PS2Driver::match_number_code(scan_code)
				}
			})
			.or_else(|| PS2Driver::match_function_code(scan_code))
			.or_else(|| PS2Driver::match_modifier_code(scan_code, auxiliary_code))
			.or_else(|| PS2Driver::match_special_code(scan_code, auxiliary_code))
			.or_else(|| {
				if self.shift_pressed() {
					PS2Driver::match_alternative_symbols(scan_code)
				} else {
					PS2Driver::match_symbol_code(scan_code)
				}
			})
			.or_else(|| PS2Driver::match_multimedia_code(scan_code, auxiliary_code))
			.or_else(|| PS2Driver::match_arrow_code(scan_code, auxiliary_code))
	}

	fn update_state(&mut self, key_code: &KeyCode, state: KeyState) {
		match *key_code {
			KeyCode::Modifier(KeyModifier::ShiftLeft) => self.shift_left_pressed = state,
			KeyCode::Modifier(KeyModifier::ShiftRight) => self.shift_right_pressed = state,
			KeyCode::Special(KeySpecial::CapsLock) => {
				if state == KeyState::Pressed {
					self.caps_lock_on = !self.caps_lock_on;
				}
			}
			_ => (),
		}
	}

	fn discard(count: u32) {
		for _ in 0..count {
			let _discard = Self::read();
		}
	}

	fn read() -> u8 {
		use x86_64::instructions::port::inb;
		const PORT: u16 = 0x60;
		unsafe { inb(PORT) }
	}

	fn shift_pressed(&self) -> bool {
		self.shift_left_pressed == KeyState::Pressed || self.shift_right_pressed == KeyState::Pressed
	}

	pub fn match_letter_code(scan_code: u8) -> Option<KeyLetter> {
		Some(match scan_code {
			0x10 => KeyLetter::Q,
			0x11 => KeyLetter::W,
			0x12 => KeyLetter::E,
			0x13 => KeyLetter::R,
			0x14 => KeyLetter::T,
			0x15 => KeyLetter::Y,
			0x16 => KeyLetter::U,
			0x17 => KeyLetter::I,
			0x18 => KeyLetter::O,
			0x19 => KeyLetter::P,
			0x1e => KeyLetter::A,
			0x1f => KeyLetter::S,
			0x20 => KeyLetter::D,
			0x21 => KeyLetter::F,
			0x22 => KeyLetter::G,
			0x23 => KeyLetter::H,
			0x24 => KeyLetter::J,
			0x25 => KeyLetter::K,
			0x26 => KeyLetter::L,
			0x2c => KeyLetter::Z,
			0x2d => KeyLetter::X,
			0x2e => KeyLetter::C,
			0x2f => KeyLetter::V,
			0x30 => KeyLetter::B,
			0x31 => KeyLetter::N,
			0x32 => KeyLetter::M,
			_ => return None,
		})
	}

	pub fn match_key_pad_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x37 => KeyPad::Asterisk,
			0x47 => KeyPad::Number(KeyNumber::Seven),
			0x48 => KeyPad::Number(KeyNumber::Eight),
			0x49 => KeyPad::Number(KeyNumber::Nine),
			0x4a => KeyPad::Minus,
			0x4b => KeyPad::Number(KeyNumber::Four),
			0x4c => KeyPad::Number(KeyNumber::Five),
			0x4d => KeyPad::Number(KeyNumber::Six),
			0x4e => KeyPad::Plus,
			0x4f => KeyPad::Number(KeyNumber::One),
			0x50 => KeyPad::Number(KeyNumber::Two),
			0x51 => KeyPad::Number(KeyNumber::Three),
			0x52 => KeyPad::Number(KeyNumber::Zero),
			0x53 => KeyPad::Dot,
			0xe0 => match auxiliary_code {
				0x1c => KeyPad::Enter,
				0x35 => KeyPad::SlashForward,
				_ => return None,
			},
			_ => return None,
		}.into())
	}

	pub fn match_number_code(scan_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x02 => KeyNumber::One,
			0x03 => KeyNumber::Two,
			0x04 => KeyNumber::Three,
			0x05 => KeyNumber::Four,
			0x06 => KeyNumber::Five,
			0x07 => KeyNumber::Six,
			0x08 => KeyNumber::Seven,
			0x09 => KeyNumber::Eight,
			0x0a => KeyNumber::Nine,
			0x0b => KeyNumber::Zero,
			_ => return None,
		}.into())
	}

	pub fn match_number_symbols(scan_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x02 => KeySymbol::Exclamation,
			0x03 => KeySymbol::At,
			0x04 => KeySymbol::Hash,
			0x05 => KeySymbol::Dollar,
			0x06 => KeySymbol::Percentage,
			0x07 => KeySymbol::AngleUp,
			0x08 => KeySymbol::Ampersand,
			0x09 => KeySymbol::Asterisk,
			0x0a => KeySymbol::BracketLeft,
			0x0b => KeySymbol::BracketRight,
			_ => return None,
		}.into())
	}

	pub fn match_function_code(scan_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x3b => KeyFunction::One,
			0x3c => KeyFunction::Two,
			0x3d => KeyFunction::Three,
			0x3e => KeyFunction::Four,
			0x3f => KeyFunction::Five,
			0x40 => KeyFunction::Six,
			0x41 => KeyFunction::Seven,
			0x42 => KeyFunction::Eight,
			0x43 => KeyFunction::Nine,
			0x44 => KeyFunction::Ten,
			0x57 => KeyFunction::Eleven,
			0x58 => KeyFunction::Twelve,
			_ => return None,
		}.into())
	}

	pub fn match_modifier_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x1d => KeyModifier::ControlLeft,
			0x2a => KeyModifier::ShiftLeft,
			0x36 => KeyModifier::ShiftRight,
			0x38 => KeyModifier::AltLeft,
			0xe0 => match auxiliary_code {
				0x1d => KeyModifier::ControlRight,
				0x38 => KeyModifier::AltRight,
				0x5b => KeyModifier::SuperLeft,
				0x5c => KeyModifier::SuperRight,
				_ => return None,
			}
			_ => return None,
		}.into())
	}

	pub fn match_special_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x01 => KeySpecial::Escape,
			0x0e => KeySpecial::Backspace,
			0x0f => KeySpecial::Tab,
			0x1c => KeySpecial::Enter,
			0x3a => KeySpecial::CapsLock,
			0x45 => KeySpecial::NumberLock,
			0x46 => KeySpecial::ScrollLock,
			0xe0 => match auxiliary_code {
				0x47 => KeySpecial::Home,
				0x49 => KeySpecial::PageUp,
				0x4f => KeySpecial::End,
				0x51 => KeySpecial::PageDown,
				0x52 => KeySpecial::Insert,
				0x53 => KeySpecial::Delete,
				0x5d => KeySpecial::Menu,
				0x5e => KeySpecial::Power,
				0x5f => KeySpecial::Sleep,
				0x63 => KeySpecial::Wake,
				_ => return None,
			},
			_ => return None,
		}.into())
	}

	pub fn match_symbol_code(scan_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x0c => KeySymbol::Minus,
			0x0d => KeySymbol::Equal,
			0x1a => KeySymbol::SquareLeft,
			0x1b => KeySymbol::SquareRight,
			0x27 => KeySymbol::Semicolon,
			0x28 => KeySymbol::SingleQuote,
			0x29 => KeySymbol::BackTick,
			0x2b => KeySymbol::SlashBackward,
			0x33 => KeySymbol::Comma,
			0x34 => KeySymbol::Dot,
			0x35 => KeySymbol::SlashForward,
			0x39 => KeySymbol::Space,
			_ => return None,
		}.into())
	}

	pub fn match_alternative_symbols(scan_code: u8) -> Option<KeyCode> {
		Some(match scan_code {
			0x0c => KeySymbol::Underscore,
			0x0d => KeySymbol::Plus,
			0x1a => KeySymbol::CurlyLeft,
			0x1b => KeySymbol::CurlyRight,
			0x27 => KeySymbol::Colon,
			0x28 => KeySymbol::DoubleQuote,
			0x29 => KeySymbol::Tilde,
			0x2b => KeySymbol::Pipe,
			0x33 => KeySymbol::AngleLeft,
			0x34 => KeySymbol::AngleRight,
			0x35 => KeySymbol::Question,
			0x39 => KeySymbol::Space,
			_ => return None,
		}.into())
	}

	pub fn match_multimedia_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		if scan_code != 0xe0 {
			return None;
		}
		Some(match auxiliary_code {
			0x10 => KeyMultimedia::Previous,
			0x19 => KeyMultimedia::Next,
			0x20 => KeyMultimedia::Mute,
			0x21 => KeyMultimedia::Calculator,
			0x22 => KeyMultimedia::Play,
			0x24 => KeyMultimedia::Stop,
			0x2e => KeyMultimedia::VolumeDown,
			0x30 => KeyMultimedia::VolumeUp,
			0x32 => KeyMultimedia::WebHome,
			0x65 => KeyMultimedia::WebSearch,
			0x66 => KeyMultimedia::WebFavourites,
			0x67 => KeyMultimedia::WebRefresh,
			0x68 => KeyMultimedia::WebStop,
			0x69 => KeyMultimedia::WebForward,
			0x6a => KeyMultimedia::WebBack,
			0x6b => KeyMultimedia::Computer,
			0x6c => KeyMultimedia::Email,
			0x6d => KeyMultimedia::MediaSelect,
			_ => return None,
		}.into())
	}

	pub fn match_arrow_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyCode> {
		if scan_code != 0xe0 {
			return None;
		}
		Some(match auxiliary_code {
			0x48 => KeyArrow::Up,
			0x4b => KeyArrow::Left,
			0x4d => KeyArrow::Right,
			0x50 => KeyArrow::Down,
			_ => return None,
		}.into())
	}

	pub fn match_print_code(scan_code: u8, auxiliary_code: u8) -> Option<KeyEvent> {
		if scan_code != 0xe0 {
			return None;
		}
		Some(match auxiliary_code {
			0x2a => KeyEvent {
				key_code: KeySpecial::PrintScreen.into(),
				state: KeyState::Pressed,
			},
			0xb7 => KeyEvent {
				key_code: KeySpecial::PrintScreen.into(),
				state: KeyState::Released,
			},
			_ => return None,
		})
	}
}
