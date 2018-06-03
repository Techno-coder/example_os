#[test]
fn test_frames_can_fit_inside_frame_store() {
	let max_frames = ::memory::generic_allocators::fixed_frame_recycler::MAX_FRAMES;
	let max_store_frames = ::structures::frame_store::MAX_FRAMES;
	assert!(max_frames <= max_store_frames);
}