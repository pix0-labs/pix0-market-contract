use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Coin, Timestamp};

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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SellOffer {

    pub token_id : String, 

    pub collection_info : Option<SimpleCollectionInfo>,

    pub owner : Addr,

    pub price : Coin, 

    pub buy_offers : Vec<BuyOffer>,

    pub allowed_direct_buy : bool,

    pub status : u8, 

    pub deal_close_type : Option<u8>, 

    pub date_created : Option<Timestamp>,
    
    pub date_updated : Option<Timestamp>,
    
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BuyOffer {

    pub owner : Addr,

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