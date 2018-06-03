use super::Evaluator;
use super::Traversal;

pub fn construct() -> Evaluator {
	let mut evaluator = Evaluator::new();
	evaluator.add_option("memory", Traversal::Evaluator(super::memory::construct()));
	evaluator
}