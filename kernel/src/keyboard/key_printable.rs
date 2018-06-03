use core::convert::Into;
use super::KeyCode;

#[derive(Debug)]
pub struct KeyCharacter {
	pub letter: KeyLetter,
	pub is_capital: bool,
}

impl KeyCharacter {
	pub fn to_char(&self) -> char {
		let mut character = self.letter.to_lower_case();
		if self.is_capital {
			character = character.to_ascii_uppercase();
		}
		character
	}
}

impl Into<KeyCode> for KeyCharacter {
	fn into(self) -> KeyCode {
		KeyCode::Character(self)
	}
}

#[derive(Debug)]
pub enum KeyLetter {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
}

impl KeyLetter {
	fn to_lower_case(&self) -> char {
		use self::KeyLetter::*;
		match *self {
			A => 'a',
			B => 'b',
			C => 'c',
			D => 'd',
			E => 'e',
			F => 'f',
			G => 'g',
			H => 'h',
			I => 'i',
			J => 'j',
			K => 'k',
			L => 'l',
			M => 'm',
			N => 'n',
			O => 'o',
			P => 'p',
			Q => 'q',
			R => 'r',
			S => 's',
			T => 't',
			U => 'u',
			V => 'v',
			W => 'w',
			X => 'x',
			Y => 'y',
			Z => 'z',
		}
	}
}

#[derive(Debug)]
pub enum KeyNumber {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
}

impl KeyNumber {
	pub fn to_char(&self) -> char {
		use self::KeyNumber::*;
		match *self {
			Zero => '0',
			One => '1',
			Two => '2',
			Three => '3',
			Four => '4',
			Five => '5',
			Six => '6',
			Seven => '7',
			Eight => '8',
			Nine => '9',
		}
	}
}

impl Into<KeyCode> for KeyNumber {
	fn into(self) -> KeyCode {
		KeyCode::Number(self)
	}
}

#[derive(Debug)]
pub enum KeySymbol {
	SquareLeft,
	SquareRight,
	SlashBackward,
	SlashForward,
	AngleLeft,
	AngleRight,
	BracketLeft,
	BracketRight,
	CurlyLeft,
	CurlyRight,
	Space,
	Minus,
	Plus,
	Equal,
	Underscore,
	Pipe,
	Question,
	At,
	Exclamation,
	Hash,
	Dollar,
	Percentage,
	AngleUp,
	Ampersand,
	Asterisk,
	Colon,
	Semicolon,
	DoubleQuote,
	SingleQuote,
	Tilde,
	BackTick,
	Comma,
	Dot,
}

impl KeySymbol {
	pub fn to_char(&self) -> char {
		use self::KeySymbol::*;
		match *self {
			SquareLeft => '[',
			SquareRight => ']',
			SlashBackward => '\\',
			SlashForward => '/',
			AngleLeft => '<',
			AngleRight => '>',
			BracketLeft => '(',
			BracketRight => ')',
			CurlyLeft => '{',
			CurlyRight => '}',
			Space => ' ',
			Minus => '-',
			Plus => '+',
			Equal => '=',
			Underscore => '_',
			Pipe => '|',
			Question => '?',
			Exclamation => '!',
			Hash => '#',
			Dollar => '$',
			Percentage => '%',
			AngleUp => '^',
			Ampersand => '&',
			Asterisk => '*',
			Colon => ':',
			Semicolon => ';',
			DoubleQuote => '"',
			SingleQuote => '\'',
			Tilde => '~',
			BackTick => '`',
			Comma => ',',
			Dot => '.',
			At => '@',
		}
	}
}

impl Into<KeyCode> for KeySymbol {
	fn into(self) -> KeyCode {
		KeyCode::Symbol(self)
	}
}
