use crate::internal::arena_skl::iterator::Splice;

pub struct Inserter<const MAX_HEIGHT: usize> {
	spl: [Splice<MAX_HEIGHT>; MAX_HEIGHT],
	height: u32,
}