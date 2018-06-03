use spin::Mutex;
use spin::MutexGuard;

pub struct Global<T> {
	identifier: &'static str,
	object: Mutex<Option<T>>,
}

impl<T> Global<T> {
	pub const fn new(identifier: &'static str) -> Global<T> {
		Global {
			identifier,
			object: Mutex::new(None),
		}
	}

	pub fn set(&self, object: T) {
		use core::ops::DerefMut;
		use core::mem::replace;
		replace(self.object.lock().deref_mut(), Some(object));
	}

	pub fn lock_direct(&self) -> MutexGuard<Option<T>> {
		self.object.lock()
	}

	#[cfg(not(debug_assertions))]
	pub fn lock(&self) -> GlobalGuard<T> {
		let guard = self.object.lock();
		if guard.is_none() { panic!("Global not initialized: {}", self.identifier); }
		GlobalGuard {
			guard,
		}
	}

	#[cfg(debug_assertions)]
	pub fn lock(&self) -> GlobalGuard<T> {
		GlobalGuard {
			guard: match self.object.try_lock() {
				Some(guard) => {
					if guard.is_none() { panic!("Global not initialized: {}", self.identifier); }
					guard
				}
				None => {
					eprintln!("Global object in contention: {}", self.identifier);
					self.object.lock()
				}
			},
		}
	}
}

pub struct GlobalGuard<'a, T: 'a> {
	guard: MutexGuard<'a, Option<T>>,
}

impl<'a, T> ::core::ops::Deref for GlobalGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &<Self as ::core::ops::Deref>::Target {
		self.guard.deref().as_ref().unwrap()
	}
}

impl<'a, T> ::core::ops::DerefMut for GlobalGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut <Self as ::core::ops::Deref>::Target {
		self.guard.deref_mut().as_mut().unwrap()
	}
}
