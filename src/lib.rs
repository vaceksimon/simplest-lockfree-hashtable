#[derive(Debug, Copy, Clone)]
struct Entry {
    key: u32,
    value: u32,
}

pub struct HashTable2 {
    array_size: u32,
    entries: Box<[Entry]>,
}

impl HashTable2 {
    pub fn new(array_size: u32) -> Self {
        assert_eq!(array_size & (array_size - 1), 0);
        let entries = vec![Entry {key: 0, value: 0, }; array_size as usize];
        let entries = entries.into_boxed_slice();

        Self {
            array_size,
            entries,
        }
    }

    pub fn get_item(&self, key: u32) -> u32 {
        todo!()
    }

    pub fn set_item(key: u32, value: u32) {
        todo!()
    }
}

/// From https://github.com/aappleby/smhasher
fn integer_hash(mut h: u32) -> u32 {
    h ^= h >> 16;
    h *= 0x85ebca6b;
    h ^= h >> 13;
    h *= 0xc2b2ae35;
    h ^= h >> 16;
    h
}
