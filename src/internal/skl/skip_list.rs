use crate::internal::base::internal::InternalKey;

pub trait SkipList<T> where T: Eq, T: Ord {
	fn get(key: InternalKey); // TODO SkipListGetResult

	fn add(key: InternalKey, val: Vec<u8>); // TODO SkipListAddResult

	fn delete(key: InternalKey); // TODO SkipListDeleteResult
}

