pub struct Node<const MAX_HEIGHT: usize> {
	key_offset: u32,
	key_size: u32,
	value_size: u32,
	alloc_size: u32,
	tower: [Links; MAX_HEIGHT]
}

impl<const MAX_HEIGHT: usize> Node<MAX_HEIGHT> {
	fn new() -> Node<MAX_HEIGHT> {
		// Node {
		// 	key_offset: 1,
		// 	key_size: 1,
		// 	value_size: 1,
		// 	alloc_size: 1,
		// 	tower: [Links{next_offset: 1, prev_offset: 5}; MAX_HEIGHT]
		// }
		todo!()
	}

	/// MaxNodeSize returns the maximum space needed for a node with the specified
	/// key and value sizes. This could overflow a uint32, which is why a uint64
	/// is used here. If a key/value overflows a uint32, it should not be added to
	/// the skiplist.
	pub fn max_node_size(key_size: u32, value_size: u32) -> u64 {
		todo!()
	}

	fn new_raw_node() {
		todo!()
	}

	fn next_offset(h: isize) {
		todo!()
	}

	fn prev_offset(h: isize) {
		todo!()
	}

	fn cas_next_offset(self, h: isize, old: u32, val: u32) {
		todo!()
	}

	fn cas_prev_offset(self, h: isize, old: u32, val: u32) {
		todo!()
	}
}

struct Links {
	next_offset: u32,
	prev_offset: u32,
}

impl Default for Links {
    fn default() -> Self {
        todo!()
    }
}

impl Copy for Links {

}

impl Clone for Links {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Links {
	fn new() -> Links { // TODO
		Links {
			next_offset: 1,
			prev_offset: 1,
		}
	}
	fn init(mut self, prev_offset: u32, next_offset: u32) {
		self.next_offset = next_offset;
		self.prev_offset = prev_offset;
	}
}
