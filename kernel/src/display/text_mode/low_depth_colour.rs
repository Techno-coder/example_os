#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LowDepthColour {
	Black,
	Blue,
	Green,
	Cyan,
	Red,
	Magenta,
	Brown,
	LightGray,
	DarkGray,
	LightBlue,
	LightGreen,
	LightCyan,
	LightRed,
	Pink,
	Yellow,
	White,
}

impl LowDepthColour {
	pub const BACKGROUND: LowDepthColour = LowDepthColour::Black;
	pub const FOREGROUND: LowDepthColour = LowDepthColour::White;
}