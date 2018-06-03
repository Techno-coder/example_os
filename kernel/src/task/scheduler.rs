use super::Thread;

pub trait Scheduler {
	/// Halts the interrupted thread and selects the next thread to run
	fn schedule_next(&mut self) -> Option<Thread>;

	/// Adds a new thread to a pool of threads to be executed
	fn schedule_new(&mut self, new_thread: Thread);
}
