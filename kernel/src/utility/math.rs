pub fn align_up_u64(number: u64, multiple: u64) -> u64 {
	align_down_u64(number + multiple - 1, multiple)
}

pub fn align_down_u64(number: u64, multiple: u64) -> u64 {
	assert!(multiple.is_power_of_two());
	number & !(multiple - 1)
}

pub fn align_up_usize(number: usize, multiple: usize) -> usize {
	align_down_usize(number + multiple - 1, multiple)
}

pub fn align_down_usize(number: usize, multiple: usize) -> usize {
	assert!(multiple.is_power_of_two());
	number & !(multiple - 1)
}

pub fn percentage(numerator: u64, denominator: u64) -> u64 {
	if denominator == 0 {
		return 0;
	}
	(100 * numerator + denominator / 2) / denominator
}
