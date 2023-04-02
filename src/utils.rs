use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


pub fn hash_to_hex<T:Hash>(t: &T) -> String {

    let h = calculate_hash(&t);
    format!("{:x}", h)
}


pub fn offer_id<T:Hash>(t: &T) -> String {
    format!("OF{}", hash_to_hex(t)).to_uppercase()
}

