#[test]
fn test_valid_size() {
	use core::mem::size_of;
	use structures::frame_store::FrameStoreNode;
	assert!(size_of::<FrameStoreNode<::memory::Frame>>() as u64 <= ::memory::Frame::SIZE);
	assert!(size_of::<FrameStoreNode<::memory::HugeFrame>>() as u64 <= ::memory::Frame::SIZE);
}
