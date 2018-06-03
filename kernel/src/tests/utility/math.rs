#[test]
fn test_align_up() {
	use utility::math::align_up_u64;
	assert_eq!(align_up_u64(0b1011_1111, 0b0100_0000), 0b1100_0000);
	assert_eq!(align_up_u64(0b0110_1011, 0b0100_0000), 0b1000_0000);
}

#[test]
fn test_align_down() {
	use utility::math::align_down_u64;
	assert_eq!(align_down_u64(0b1011_1111, 0b0100_0000), 0b1000_0000);
	assert_eq!(align_down_u64(0b0110_1011, 0b0100_0000), 0b0100_0000);
}
