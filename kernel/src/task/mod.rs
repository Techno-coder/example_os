pub use self::functions::SCHEDULER;
pub use self::scheduler::Scheduler;
pub use self::thread::Thread;

pub mod scheduler;
pub mod schedulers;
pub mod thread;
pub mod functions;
pub mod loaders;

// loaders/flat_binary contains a function to load a "flat_binary"
// Flat binaries only contain machine code and nothing else
// You can generate one with NASM:
//      nasm -f bin <assembly_file>
// You can then load the file via the boot disk
