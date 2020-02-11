use crate::BitmapIndex;

use super::{Field, Bitmap, BITMAP_LENGTH};

use std::collections::HashMap;

impl BitmapIndex{
    pub fn new() -> BitmapIndex {
        BitmapIndex {items: HashMap::new()}
    }

    /// Add value
    /// ``` rust
    /// use BitmapIndex;
    /// let mut bi = BitmapIndex::new();
    /// bi.insert(key: "123", value: 4)
    /// 
    /// 
    pub fn insert<T>(&mut self, key: u64, value: u64) -> bool {
        let mut _field = match self.items.get_mut(&value) {
            Some(field) => field,
            None => {
                self.items.insert(value, Field::new());
                self.items.get_mut(&value).unwrap()
            }
        };
        _field.insert(key)
    }

    pub fn get(&mut self) -> bool {
        true
    }
}    


impl Field {
    pub fn new() -> Field {
        Field {len: 0, data: Bitmap::new([0; BITMAP_LENGTH])}
    }

    pub fn insert(&mut self, key: u64) -> bool {
        // Baic implementation
        self.data[key as usize] = 1;
        true
    }
}

/*
Method that must be implemented: 
 - Insert => Insert value at the end
 - Update
 - Delete (set bool to false)
 - Get => Get the value at a specific index
 - Return the list of value True for a specific key

*/