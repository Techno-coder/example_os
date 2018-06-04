use alloc::VecDeque;
use super::Thread;

// The simplest scheduler possible
// The next thread is the thread pushed on earliest
// New threads are pushed to the back of the queue

pub struct RoundRobin {
	threads: VecDeque<Thread>,
}

impl RoundRobin {
	pub fn new() -> RoundRobin {
		RoundRobin {
			threads: VecDeque::new(),
		}
	}
}

impl super::Scheduler for RoundRobin {
	fn schedule_next(&mut self) -> Option<Thread> {
		self.threads.pop_front()
	}

	fn schedule_new(&mut self, new_thread: Thread) {
		self.threads.push_back(new_thread);
	}
}