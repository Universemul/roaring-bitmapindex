mod inherent;

use std::collections::HashMap;

const BITMAP_LENGTH: usize = 4096;

pub type Bitmap = Box<[u64; BITMAP_LENGTH]>;
pub type Key = String;

pub struct Field {
    pub len: u64, // Define the number of keys inside data
    pub data: Bitmap
}

pub struct BitmapIndex {
    pub items: HashMap<u64, Field>
}  

/*
e.g => BitmapIndex on a field called "Job"
key is the id of my document, value 
let b_index = BitmapIndex::new();
b_index.insert(key: 123, value: "Developer");
b_index.remove(key: 123, value: "Developer");
b_index.contains(value: "Developer"); => Must return all document wtih the job "Developer"
*/