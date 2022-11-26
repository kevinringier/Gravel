// use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

// /// TODO
// /// I think the Arena is the backing byte array used in the skiplist
// /// n: number of bytes allocated by the arena or the current pointer location in the arena
// /// buf: is the backing buffer
// pub struct Arena {
// 	n: AtomicU64, // KR: this will likely need to be wrapped in an atomic
// 	buf: [u8; 10], // TODO is this a predetermined size? pretty sure this is predetermined based on a max config, but might be change at runtime which means it should be a vector. If this only uses const values, then we will know at compile time and can use arr syntax
// }

// pub enum ArenaErrors {
// 	ArenaFullError,
// }

// impl Arena {
// 	/// new allocates a new arena using the specified buffer as the backing
// 	/// store.
// 	pub fn new(buf: [u8;10]) -> Arena { // in pebble this is a pointer to an Arena and it returns the reference
// 		// Don't store data at position 0 in order to reserve offset=0 as a
// 		// kind of nil pointer. KR: Why do we need this?
// 		Arena {
// 			n: AtomicU64::new(1),
// 			buf, // this will move ownership, should this be shared or do I copy?
// 				 // I'm assuming this is shared but I need to understand more about arena allocation
// 				 // But it says it will be lock-free. hmm
// 		}
// 	}

// 	/// size returns the number of bytes allocated by the arena
// 	pub fn size(self) -> u32 {
// 		let size = self.n.load(Relaxed) as u32;
		
// 		if size > u32::MAX {
// 			return u32::MAX
// 		}

// 		size
// 	}

// 	/// capacity returns the capacity of the arena
// 	pub fn capacity(self) -> u32 {
// 		self.buf.len() as u32
// 	}

// 	pub fn allocate(self, size: u32, align: u32, overflow: u32) -> Result<(u32, u32), ArenaErrors> { // (offset, padded)
// 		// Verify the arena isn't already full.
// 		let orig_size = self.n.load(Relaxed);

// 		if orig_size as usize > self.buf.len() {
// 			return Err(ArenaErrors::ArenaFullError);
// 		}

// 		// Pad the allocation with enough bytes to ensure the requested alginment.
// 		let padded = size + align;

// 		let new_size = self.n.load(Relaxed) as u32 + padded;

// 		if (new_size + overflow) as usize > self.buf.len() {
// 			return Err(ArenaErrors::ArenaFullError);
// 		}

// 		let offset = (new_size - padded + align) & (!align);
// 		// bitwise ops may be wrong but using the following for reference:
// 		// Use bit clear AND NOT &^ to get the bits that are in 3 AND NOT 6 (order matters)
// 		// 3      = 00000011
// 		// 6      = 00000110
// 		// 3 &^ 6 = 00000001 = 1
// 		Ok((offset, padded))
// 	}

// 	pub fn get_bytes(self, offset: u32, size: u32) -> Vec<u8> {
// 		if offset == 0 {
// 			return Vec::new(); // KR: this is not okay, we should return a typed value indicating what this means. Search for how `nil` is used in pebble and construct appropriate return type for this
// 		}
// 		todo!()
// 		//return a.buf[offset : offset+size : offset+size] Go what does this mean?
// 	}

// 	pub fn get_pointer(offset: u32) {todo!()}

// 	pub fn get_pointer_offset() {todo!()}
// }

use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::{mem, slice};

pub const K_BLOCK_SIZE: usize = 4096;

#[derive(Default)]
pub struct ArenaInner {
	alloc_ptr: AtomicPtr<u8>,
	remaining_bytes: AtomicUsize,
	memory_usage: AtomicUsize,
	blocks: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl ArenaInner {
	fn new() -> Self {
		Self::default()
	}

	/// remaining_bytes returns the number of remaining bytes in the arena
	fn remaining_bytes(&self) -> usize {
		//TODO understand atomic lock ordering https://en.cppreference.com/w/cpp/atomic/memory_order#Release-Acquire_ordering
		self.remaining_bytes.load(Ordering::Aquire)
	}

	/// sub_remaining_bytes subtracts bytes from this arena's remaining bytes
	fn sub_remaining_bytes(&self, bytes: usize) {
		self.remaining_bytes.fetch_sub(bytes, Ordering::Release);
	}

	/// alloc_ptr returns the current index where memory will be allocated
	fn alloc_ptr(&self) -> *mut u8 {
		self.alloc_ptr.load(Ordering::Acquire)
	}
	
	/// add_alloc_ptr increments the current index by the number of bytes and stores it
	fn add_alloc_ptr(&self, bytes: usize) {
		let p = self.alloc_ptr();
		self.alloc_ptr
		.store(unsafe {p.add(bytes)}, Ordering::Release);
	}

    fn alloc_fallback(&self, bytes: usize) -> *mut u8 {
        if bytes > K_BLOCK_SIZE / 4 {
            // Object is more than a quarter of our block size.  Allocate it separately
            // to avoid wasting too much space in leftover bytes.
            return self.allocate_new_block(bytes);
        }

        // We waste the remaining space in the current block.
        self.alloc_ptr
            .store(self.allocate_new_block(K_BLOCK_SIZE), Ordering::Release);
        self.remaining_bytes.store(K_BLOCK_SIZE, Ordering::Release);

        let result = self.alloc_ptr();
        self.add_alloc_ptr(bytes);
        self.sub_remaining_bytes(bytes);
        result
    }

	fn allocate_new_block(&self, bytes: usize) -> *mut u8 {
        let mut v = vec![0; bytes];

        let result = v.as_mut_ptr();
        self.blocks.lock().unwrap().push(v);
        self.memory_usage.store(
            self.memory_usage() + bytes + mem::size_of::<usize>(),
            Ordering::Release,
        );
        unsafe { mem::transmute(result) }
    }

	/// memory_usage returns the amount of memory used in arena
	fn memory_usage(&self) -> usize {
        self.memory_usage.load(Ordering::Acquire)
    }
}