use crate::internal::base::internal::InternalKey;

/// InternalIterator iterates over a DB's key/value pairs in key order.
/// The returned keys are InternalKeys composed of the user-key, a sequence
/// number and a key kind. In forward iteration, key/value pairs for
/// identical user-keys are returned in descending sequence order. In
/// reverse iteration, key/value pairs for identical user-keys are returned
/// in ascending sequence order.
///
/// InternalIterators provide 5 absolute positioning methods and 2 relative
/// positioning methods. The absolute positioning methods are:
///
/// - SeekGE
/// - SeekPrefixGE
/// - SeekLT
/// - First
/// - Last
///
/// The relative positioning methods are:
///
/// - Next
/// - Prev
///
/// The relative positioning methods can be used in conjunction with an of
/// the absolute positioning methods with one exception: SeekPrefixGE does
/// not support reverse iteration via Prev. It is undefined to call relative
/// positioning methods without ever calling an absolute positioning method.
///
/// InternalIterators can optionally implement a prefix iteration mode. This
/// mode is entered by calling SeekPrefixGE and exited by any other other
/// absolute positioning method (SeekGE, SeekLT, First, Last). When in prefix
/// iteration mode, a call to Next will advance to the next key which has the
/// same "prefix" as the one supplied to SeekPrefixGE. Note that "prefix" in
/// this context is not a strict byte prefix, but defined by byte equality for
/// the result of the Comparer.Split method. An InternalIterator is not required
/// to support prefix iteration mode, and can implement SeekPrefixGE by
/// forwarding to SeekGE.
///
/// Bound, [lower, upper), can be set on iterators, either using the SetBounds()
/// function in the interface, or in the implementation specific ways during
/// iterator creation. The forward positioning routines (SeekGE, First, and Next)
/// only check the upper bound. The reverse positioning routines (SeekLT, Last, 
/// and Prev) only check the lower bound. It is up to the caller to ensure that
/// the forward positioning routines respect the lower bound and the reverse
/// positioning routines respect the upper bound (i.e. calling SeekGE instead of
/// First if there is a lower bound, and SeekLT instead of Last if there is an
/// upper bound). This imposition is done in order to elevate that enforcement to
/// the caller, a higher-level Iterator, rather than having it duplicated in every
/// InternalIterator implementation.
///
/// Additionally, the caller needs to ensure that SeekGE/SeekPrefixGE are not
/// called with a key > the upper bound, and SeekLT is not called with a key <
/// the lower bound. InternalIterator implementations are required to respect
/// the iterator bounds, never returning records outside of the bounds with one
/// exception: an iterator may generate synthetic RANGEDEL marker records. See
/// levelIter.syntheticBoundary for the sole existing example of this behavior.
/// Specifically, levelIter can return synthetic keys whose user key is equal to
/// the lower/upper bound.
///
/// The bounds provided to an internal iterator must remain valid until a
/// subsequent call to SetBounds has returned. This requirement exists so that
/// iterator implementations may compare old and new bounds to apply low-level
/// optimizations. The pebble.Iterator satisfies this requirement by maintaining
/// two bound buffers and switching between them.
///
/// An iterator must be closed after use, but it is not necessary to read an
/// iterator until exhaustion.
///
/// An iterator is not goroutine-safe, but it is safe to use multiple iterators
/// concurrently, either in separate goroutines or switching between the
/// iterators in a single goroutine.
///
/// It is also safe to use an iterator concurrently with modifying its
/// underlying DB, if that DB permits modification. However, the resultant
/// key/value pairs are not guaranteed to be a consistent snapshot of that DB
/// at a particular point in time.
///
/// InternalIterators accumulate errors encountered during operation, exposing
/// them through the Error method. All of the absolute positioning methods
/// reset any accumulated error before positioning. Relative positioning
/// methods return without advancing if the iterator has accumulated an error.
///
/// nilv == shorthand for LazyValue{}, which represents a nil value.
// TODO I think we should break interfaces apart. A particular iterator shouldnt be forced to implement if not necessary.
pub trait InternalIterator {
	fn seek_ge(key: [u8], flags: SeekGEFlags) -> (InternalKey, [u8]);
	
}

type SeekGEFlags = u8; // TODO can this be an enum