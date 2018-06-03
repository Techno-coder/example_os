pub struct PseudoRandomGenerator {
	next: u64,
}

impl PseudoRandomGenerator {
	pub fn new(seed: u64) -> PseudoRandomGenerator {
		PseudoRandomGenerator {
			next: seed,
		}
	}

	pub fn next(&mut self) -> u64 {
		self.next = self.next.wrapping_mul(1103515245) + 12345;
		(self.next / 65536)
	}
}
