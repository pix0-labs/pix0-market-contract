use cosmwasm_std::{Deps, DepsMut};
use crate::indexes::COLLECTION_INDEX_STORE;
use crate::state::{SimpleCollectionInfo, CollectionIndex};
use crate::utils::to_collection_id;

#[allow(dead_code)]
pub (crate) fn collection_exists( deps: &Deps, collection_info :
SimpleCollectionInfo) -> Option<CollectionIndex> {

    
    let _key = to_collection_id(collection_info);

    let loaded_coll = COLLECTION_INDEX_STORE.may_load(deps.storage, _key);
    
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

#[allow(dead_code)]
pub (crate) fn save_collection_index(deps: DepsMut, collection_info :
    Option<SimpleCollectionInfo>)   {
    
    if collection_info.is_some() {

        let collection_info = collection_info.unwrap();

        let collection_index = collection_exists(&deps.as_ref(),
            collection_info.clone());

        let _key = to_collection_id(collection_info.clone());

        let mut new_collection_index = CollectionIndex {

            collection_info : collection_info,
            id : _key.clone(), 
            number_of_sell_offers : 1, 
        };

        if collection_index.is_some() {
            new_collection_index = collection_index.unwrap();
            new_collection_index.number_of_sell_offers += 1;
        }

       
        //ignore the error
        let _ = COLLECTION_INDEX_STORE.save(deps.storage, _key, &new_collection_index);

    }

}


#[allow(dead_code)]
pub (crate) fn remove_sell_offer_from_index(deps: DepsMut, collection_info :
    Option<SimpleCollectionInfo>)   {
    
    if collection_info.is_some() {

        let collection_info = collection_info.unwrap();

        let collection_index = collection_exists(&deps.as_ref(),
            collection_info.clone());

        if collection_index.is_some() {
            let mut collection_index = collection_index.unwrap();
            collection_index.number_of_sell_offers -= 1;
            let _key = to_collection_id(collection_info);

            if collection_index.number_of_sell_offers == 0 {

                let _ = COLLECTION_INDEX_STORE.remove(deps.storage, _key);
            }
            else {

                let _ = COLLECTION_INDEX_STORE.save(deps.storage, _key, &collection_index);
            }
        }
   
    }

}



