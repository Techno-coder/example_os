use core::convert::Into;
use super::key_printable::KeyNumber;
use super::KeyCode;

#[derive(Debug)]
pub enum KeyFunction {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Eleven,
	Twelve,
}

impl Into<KeyCode> for KeyFunction {
	fn into(self) -> KeyCode {
		KeyCode::Function(self)
	}
}

#[derive(Debug)]
pub enum KeyPad {
	Number(KeyNumber),
	Asterisk,
	Minus,
	Plus,
	Dot,
	Enter,
	SlashForward,
}

impl Into<KeyCode> for KeyPad {
	fn into(self) -> KeyCode {
		KeyCode::KeyPad(self)
	}
}

#[derive(Debug)]
pub enum KeyModifier {
	ControlLeft,
	ControlRight,
	AltLeft,
	AltRight,
	ShiftLeft,
	ShiftRight,
	SuperLeft,
	SuperRight,
}

impl Into<KeyCode> for KeyModifier {
	fn into(self) -> KeyCode {
		KeyCode::Modifier(self)
	}
}

#[derive(Debug)]
pub enum KeySpecial {
	Backspace,
	CapsLock,
	Tab,
	PageUp,
	PageDown,
	Escape,
	Enter,
	Home,
	End,
	Insert,
	Delete,
	Pause,
	PrintScreen,
	Menu,
	Power,
	Sleep,
	Wake,
	NumberLock,
	ScrollLock,
}

impl Into<KeyCode> for KeySpecial {
	fn into(self) -> KeyCode {
		KeyCode::Special(self)
	}
}

#[derive(Debug)]
pub enum KeyMultimedia {
	Previous,
	Next,
	Mute,
	Calculator,
	Play,
	Stop,
	VolumeDown,
	VolumeUp,
	WebHome,
	WebSearch,
	WebFavourites,
	WebRefresh,
	WebStop,
	WebForward,
	WebBack,
	Computer,
	Email,
	MediaSelect,
}

impl Into<KeyCode> for KeyMultimedia {
	fn into(self) -> KeyCode {
		KeyCode::Multimedia(self)
	}
}

#[derive(Debug)]
pub enum KeyArrow {
	Up,
	Down,
	Left,
	Right,
}

impl Into<KeyCode> for KeyArrow {
	fn into(self) -> KeyCode {
		KeyCode::Arrow(self)
	}
}
