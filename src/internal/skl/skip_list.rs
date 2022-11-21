use crate::internal::base::internal::InternalKey;

pub trait SkipList<T> where T: Eq, T: Ord {
	/// height returns the height of the highest tower within any of the
	/// nodes that have ever been allocated as part of this skiplist
	fn height(self) -> u32;

	fn add(key: InternalKey, val: Vec<u8>, test: i32); // TODO SkipListAddResult?

	fn delete(key: InternalKey); // TODO SkipListDeleteResult?
}