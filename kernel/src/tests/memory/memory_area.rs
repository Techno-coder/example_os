#[test]
fn test_memory_area() {
	use memory::MemoryArea;
	use memory::PhysicalAddress;

	{
		let first = MemoryArea::new(PhysicalAddress::new(100), 100);
		let other = MemoryArea::new(PhysicalAddress::new(150), 100);

		let overlap = first.overlap(&other).unwrap();
		assert_eq!(overlap.start_address().raw(), 150);
		assert_eq!(overlap.end_address().raw(), 200);
		assert_eq!(overlap.size(), 50);
	}
}