use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{SellOffer, BuyOffer};
use cosmwasm_std::{Addr, Coin};
use pix0_contract_common::state::{Fee, Contract};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    UpdateContractInfo {

        fees : Option<Vec<Fee>>, 

        treasuries : Option<Vec<Addr>>,

        contracts : Option<Vec<Contract>>, 

        log_last_payment : Option<bool>, 
    },

    CreateSellOffer {

        offer : SellOffer,
    },

    UpdateSellOffer {

        offer : SellOffer,
    },


    CancelSellOffer {

        token_id : String,
    },

    CreateBuyOffer {

        buy_offer : BuyOffer,

        sell_offer_id : String,

    },

    UpdateBuyOffer {

        buy_offer : BuyOffer,

        sell_offer_id : String,

    },

    AcceptBuyOffer {

        buy_offer_by : Addr,

        sell_offer_id : String,

    },


    CancelBuyOffer {

        sell_offer_id : String,

    },

    DirectBuy {

        sell_offer_id : String,

    },


    TestTransferToEscrow {
        coin : Coin,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {

    GetSellOffersOf {
        owner : Addr, 

        status : Option<u8>,

        start : Option<u32>,

        limit : Option<u32>,
    },

    GetSellOfferById {

        offer_id : String,
    },

    GetBalanceOfEscrow {

        denom : String, 
    },

    GetBuyOffersOf {
        owner : Addr, 

        accepted : Option<bool>,

        start : Option<u32>,

        limit : Option<u32>,
    },


    GetBuyOffersBy {
        sell_offer_id : String , 

        accepted : Option<bool>,

        start : Option<u32>,

        limit : Option<u32>,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SellOffersWithParamsResponse {
    
    pub offers : Vec<SellOffer>,

    pub total : Option<u32>,

    pub start : Option<u32>,

    pub limit : Option<u32>,
}



impl SellOffersWithParamsResponse {

    pub fn empty_response() -> Self {

        SellOffersWithParamsResponse {
            offers: vec![],
            total : None,
            start : None,
            limit : None, 
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BuyOffersWithParamsResponse {
    
    pub offers : Vec<BuyOffer>,

    pub total : Option<u32>,

    pub start : Option<u32>,

    pub limit : Option<u32>,
}


impl BuyOffersWithParamsResponse {

    pub fn empty_response() -> Self {

        BuyOffersWithParamsResponse {
            offers: vec![],
            total : None,
            start : None,
            limit : None, 
        }
    }
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SellOfferResponse {

    pub offer : Option<SellOffer>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {

    pub amount : Option<Coin>,
}