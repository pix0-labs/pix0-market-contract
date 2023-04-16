use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Coin, Timestamp};
use pix0_market_handlers::state::SimpleCollectionInfo;

pub const SELL_STATUS_NEW : u8 = 1;

pub const SELL_STATUS_CLOSED : u8 = 2;

pub const DEAL_CLOSED_OFFER_ACCEPTED : u8 = 1;

pub const DEAL_CLOSED_AT_DIRECT_BUY : u8 = 2;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionIndex {

    pub collection_info : SimpleCollectionInfo,

    pub id : String, 

    pub number_of_sell_offers : u32, 

    pub date_created : Option<Timestamp>,
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


