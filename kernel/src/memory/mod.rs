pub use self::address::PhysicalAddress;
pub use self::frame::Frame;
pub use self::frame::FrameIter;
pub use self::frame::FrameLike;
pub use self::frame::HugeFrame;
pub use self::frame_allocator::FrameLikeAllocator;
pub use self::functions::FRAME_ALLOCATOR;
pub use self::generic_allocators::GenericAllocator;
pub use self::memory_area::MemoryArea;

pub mod frame_allocators;
pub mod huge_frame_allocators;
pub mod frame_allocator;
pub mod frame;
pub mod address;
pub mod memory_area;
pub mod generic_allocators;
pub mod functions;
