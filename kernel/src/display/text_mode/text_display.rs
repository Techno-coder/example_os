use super::LowDepthColour;
use super::Position;

pub trait TextDisplay {
	fn width(&self) -> super::Width;
	fn height(&self) -> super::Height;

	fn set_cell(&mut self, position: &Position, character: super::Character);
	fn set_background(&mut self, position: &Position, colour: &LowDepthColour);
	fn set_foreground(&mut self, position: &Position, colour: &LowDepthColour);

	fn set_cursor(&mut self, position: &Position);
}