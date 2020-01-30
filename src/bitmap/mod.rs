use std::collections::HashMap;

const BITMAP_LENGTH: usize = 1024;

pub type Bitmap = Box<[u64; BITMAP_LENGTH]>;

pub struct BitmapIndex {
    pub len: u64, // Define the number of keys inside data
    pub data: HashMap<String, Bitmap>
}

/*
Method that must be implemented: 
 - Insert => Insert value at the end
 - Update
 - Delete (set bool to false)
 - Get => Get the value at a specific index
 - Return the list of value True for a specific key

*/