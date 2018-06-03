pub use self::boot_allocator::BootAllocator;
pub use self::fixed_frame_recycler::FixedFrameRecycler;
pub use self::frame_recycler::FrameRecycler;
pub use self::global_frame_allocator::GlobalFrameAllocator;
pub use self::post_boot_allocator::PostBootAllocator;
use super::FrameLikeAllocator;

pub mod boot_allocator;
pub mod global_frame_allocator;
pub mod frame_recycler;
pub mod fixed_frame_recycler;
pub mod post_boot_allocator;

pub trait GenericAllocator: FrameLikeAllocator<super::Frame> + FrameLikeAllocator<super::HugeFrame> {}
