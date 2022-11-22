use crate::internal::arena_skl::node::Node;

pub struct Splice<const MAX_HEIGHT: usize> {
	prev: Node<MAX_HEIGHT>,
	next: Node<MAX_HEIGHT>,
}