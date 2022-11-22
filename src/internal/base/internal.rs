/// InternalKey is a key used for the in-memory and on-disk partial DBs
/// that make up gravel DB.
///
/// It consists of the user key (as given by the code that uses gravel)
/// followed by 8-bytes of metadata:
///		- 1 byte for the type of internalkey: delete or set,
///		- 7 bytes for a u56 sequence number, in little-endian format.
pub struct InternalKey {
	user_key: Vec<u8>,
	trailer: u64,
}

// TODO implement key encoder/decoder
//https://serde.rs/
