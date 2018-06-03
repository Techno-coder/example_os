pub use self::huge_boot_bump_allocator::HugeBootBumpAllocator;
pub use self::huge_bump_allocator::HugeBumpAllocator;
use super::FrameLikeAllocator;

pub mod huge_boot_bump_allocator;
pub mod huge_bump_allocator;