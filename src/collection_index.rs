use cosmwasm_std::Deps;
use crate::{indexes::COLLECTION_INDEX, state::SimpleCollectionInfo};
use crate::utils::to_collection_id;

#[allow(dead_code)]
pub (crate) fn collection_exists( deps: &Deps, collection_info :
SimpleCollectionInfo) -> Option<SimpleCollectionInfo> {

    
    let _key = to_collection_id(collection_info);

    let loaded_coll = COLLECTION_INDEX.may_load(deps.storage, _key);
    
    match loaded_coll {

        Ok (c) => {
            if c.is_some() {
                Some(c.unwrap())
            }
            else {
                None
            }
        },

        Err(_)=> None, 
    }

}

