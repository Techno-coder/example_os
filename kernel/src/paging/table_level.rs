pub trait TableLevel {}

pub enum Level4 {}

pub enum Level3 {}

pub enum Level2 {}

pub enum Level1 {}

impl TableLevel for Level4 {}

impl TableLevel for Level3 {}

impl TableLevel for Level2 {}

impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
	type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
	type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
	type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
	type NextLevel = Level1;
}
