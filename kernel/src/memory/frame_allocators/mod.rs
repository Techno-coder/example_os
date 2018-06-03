pub use self::huge_frame_divider::HugeFrameDivider;
pub use self::tiny_allocator::TinyAllocator;
use super::FrameLikeAllocator;

pub mod huge_frame_divider;
pub mod tiny_allocator;