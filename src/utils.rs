use pix0_market_handlers::state::SimpleCollectionInfo;
use pix0_market_handlers::utils::hash_to_hex;


pub fn to_unique_token_id(contract_addr : String, token_id : String )->String{

    format!("{}", hash_to_hex(&(contract_addr, token_id)))
}


pub fn to_collection_id(collection_info : SimpleCollectionInfo) -> String {

    format!("{}", hash_to_hex(&(collection_info.owner, collection_info.collection_name,
    collection_info.collection_symbol))).to_uppercase()

}