use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Coin, Timestamp};
use crate::utils::offer_id;

pub const SELL_STATUS_NEW : u8 = 1;

pub const SELL_STATUS_CLOSED : u8 = 2;

pub const DEAL_CLOSED_OFFER_ACCEPTED : u8 = 1;

pub const DEAL_CLOSED_AT_DIRECT_BUY : u8 = 2;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimpleCollectionInfo {

    pub owner : Addr,

    pub collection_name : String,

    pub collection_symbol : String, 

    pub category : Option<String>,
}

impl SimpleCollectionInfo {

    pub fn default () -> Self {

        SimpleCollectionInfo {
            owner : Addr::unchecked(""),
            collection_name : String::from(""),
            collection_symbol : String::from(""),
            category : Some(String::from("")), 
        }
    } 
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionIndex {

    pub collection_info : SimpleCollectionInfo,

    pub number_of_sell_offers : u32, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SellOffer {

    pub token_id : String, 

    pub owner : Addr,

    // The NFT contract address
    pub contract_addr : String, 

    pub offer_id : Option<String>,

    pub collection_info : Option<SimpleCollectionInfo>,

    pub price : Coin, 

    pub allowed_direct_buy : bool,

    pub status : u8, 

    pub deal_close_type : Option<u8>, 

    pub date_created : Option<Timestamp>,
    
    pub date_updated : Option<Timestamp>,
    
}


impl SellOffer {

    pub fn to_offer_id(&self) -> Option<String> {

        let key = (self.owner.clone(), self.contract_addr.clone(), self.token_id.clone());

        return Some(offer_id(&key));
        
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BuyOffer {

    pub owner : Addr,

    pub sell_offer_id : String,
    
    pub price : Coin, 

    pub accepted : bool, 

    pub date_created : Option<Timestamp>,
    
    pub date_updated : Option<Timestamp>,
    
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PurchaseHistory {

    pub buyer : Addr,

    pub token_id : String, 

    pub at_price : Coin, 

    pub date_created : Option<Timestamp>,
   
}


