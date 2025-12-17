use murmur3::murmur3_32;
use std::io::Cursor;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Debug)]
struct Entry {
    key: AtomicU32,
    value: AtomicU32,
}

/// Rust implementation of ["The World's Simplest Lock-Free Hash Table"](https://preshing.com/20130605/the-worlds-simplest-lock-free-hash-table/)
///
/// # Example
/// ```
/// use simplest_lockfree_hashtable::HashTable2;
///
/// let size = 4;
/// let hash_table = HashTable2::new(size);
///
/// hash_table.set_item(1, 1);
/// hash_table.set_item(1, 2);
/// hash_table.set_item(10, 10);
///
/// assert_eq!(2, hash_table.get_item(1));
/// assert_eq!(10, hash_table.get_item(10));
/// ```
#[derive(Debug)]
pub struct HashTable2 {
    array_size: u32,
    entries: Box<[Entry]>,
}

impl HashTable2 {
    pub fn new(array_size: u32) -> Self {
        assert_eq!(array_size & (array_size - 1), 0);
        let entries = init_array(array_size);

        Self {
            array_size,
            entries,
        }
    }

    pub fn get_item(&self, key: u32) -> u32 {
        assert_ne!(key, 0);

        let idx = integer_hash(key);
        for mut idx in idx.. {
            idx &= self.array_size - 1;

            let probed_key = self.entries[idx as usize].key.load(Relaxed);
            if probed_key == key {
                return self.entries[idx as usize].value.load(Relaxed);
            }
            if probed_key == 0 {
                return 0;
            }
        }
        panic!("cannot find key {}", key);
    }

    pub fn set_item(&self, key: u32, value: u32) {
        assert_ne!(key, 0);
        assert_ne!(value, 0);

        let idx = integer_hash(key);
        for mut idx in idx.. {
            idx &= self.array_size - 1;

            // Load the key that was there.
            let probed_key = self.entries[idx as usize].key.load(Relaxed);
            if probed_key != key {
                // The entry was either free, or contains another key.
                if probed_key != 0 {
                    continue; // Usually, it contains another key. Keep probing.
                }

                // The entry was free. Now let's try to take it using a CAS.
                let prev_key = self.entries[idx as usize]
                    .key
                    .compare_exchange(0, key, Relaxed, Relaxed);
                if prev_key.is_err() {
                    continue; // Another thread just stole it from underneath us.
                }

                // Either we just added the key, or another thread did.
            }

            // Store the value in this array entry.
            self.entries[idx as usize].value.store(value, Relaxed);
            return;
        }
    }
}

fn integer_hash(h: u32) -> u32 {
    murmur3_32(&mut Cursor::new(h.to_le_bytes()), 0).unwrap()
}

fn init_array(array_size: u32) -> Box<[Entry]> {
    let mut vector = vec![];
    for _ in 0..array_size {
        vector.push(Entry {
            key: AtomicU32::new(0),
            value: AtomicU32::new(0),
        })
    }
    vector.into_boxed_slice()
}
