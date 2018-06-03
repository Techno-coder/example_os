use super::key_other::*;
use super::key_printable::*;

#[derive(Debug)]
pub struct KeyEvent {
	pub key_code: KeyCode,
	pub state: KeyState,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyState {
	Pressed,
	Released,
}

#[derive(Debug)]
pub enum KeyCode {
	Character(KeyCharacter),
	Number(KeyNumber),
	Function(KeyFunction),
	KeyPad(KeyPad),
	Symbol(KeySymbol),
	Modifier(KeyModifier),
	Special(KeySpecial),
	Multimedia(KeyMultimedia),
	Arrow(KeyArrow),
}
