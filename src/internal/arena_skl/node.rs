struct Node<const MAX_HEIGHT: usize> {
	key_offset: u32,
	key_size: u32,
	value_size: u32,
	alloc_size: u32,
	tower: [Links; MAX_HEIGHT]
}

impl<const MAX_HEIGHT: usize> Node<MAX_HEIGHT> {
	fn new() -> Node<MAX_HEIGHT> {
		Node {
			key_offset: 1,
			key_size: 1,
			value_size: 1,
			alloc_size: 1,
			tower: [Links{next_offset: 1, prev_offset: 5}; MAX_HEIGHT]
		}
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
