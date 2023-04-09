use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::state::SimpleCollectionInfo;

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

pub fn to_unique_token_id(contract_addr : String, token_id : String )->String{

    format!("{}", hash_to_hex(&(contract_addr, token_id)))
}



pub fn to_collection_id(collection_info : SimpleCollectionInfo) -> String {

    format!("{}", hash_to_hex(&(collection_info.owner, collection_info.collection_name,
    collection_info.collection_symbol))).to_uppercase()

}