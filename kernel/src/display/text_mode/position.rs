use super::Height;
use super::Width;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
	pub column: Width,
	pub row: Height,
}

impl Default for Position {
	fn default() -> Self {
		Self { column: 0, row: 0 }
	}
}

impl<'a, 'b> ::core::ops::Add<&'b Position> for &'a Position {
	type Output = Position;

	fn add(self, rhs: &'b Position) -> <Self as ::core::ops::Add<&'b Position>>::Output {
		Position {
			column: self.column + rhs.column,
			row: self.row + rhs.row,
		}
	}
}
