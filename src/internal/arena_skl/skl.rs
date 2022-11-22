use crate::internal::arena_skl::arena::Arena;
use crate::internal::arena_skl::node::Node;
use crate::internal::base::internal::InternalKey;

const MAX_HEIGHT: usize = 20;

struct ArenaSkipList {
	arena: Arena,
	head: Node<MAX_HEIGHT>,
	tail: Node<MAX_HEIGHT>,
	height: u32, // Current height. 1 <= height <= MAX_HEIGHT.
}

impl ArenaSkipList {

	pub fn new_node<T: Eq + Ord>(self, key: T, value: Vec<u8>) -> (Node<MAX_HEIGHT>, u32/*height*//*, error*/) { // TODO ResultType
		todo!()
	}

	pub fn random_height(self) -> u32 {
		todo!()
	}



}

impl ArenaSkipList {
	// This set of methods were taken from node.go in arenaskl. I thought they made more sense to be in the skiplist implementation
	fn get_key_bytes(self, arena: &Arena, node: &Node) {
		todo!()
	}

	fn get_value(self, arena: &Arena, node: &Node) {
		todo!()
	}

	
}