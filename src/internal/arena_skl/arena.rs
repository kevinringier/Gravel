
/// TODO
/// I think the Arena is the backing byte array used in the skiplist
/// n: number of bytes allocated by the arena
/// buf: is the backing buffer
struct Arena {
	n: u64,
	// buf: [u8], // TODO is this a predetermined size?
}

impl Arena {
	pub fn new(buf: &[u8]) -> Arena {
		todo!();
		// this can't return a reference. This function owns the value and once this scope ends the value will be freed, thus creating a dangling pointer. Of course this can't happen in rust but the compiler will complain
	}

	/// size returns the number of bytes allocated by the arena
	pub fn size(self) -> u32 {
		1
	}

	pub fn capacity(self) -> u32 {
		// return uint32(len(a.buf))
		1
	}
}