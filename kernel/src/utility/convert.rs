pub fn as_u8_slice<T>(slice: &[T]) -> &[u8] {
	unsafe {
		::core::slice::from_raw_parts(slice.as_ptr() as *const u8,
		                              slice.len() * ::core::mem::size_of::<T>())
	}
}