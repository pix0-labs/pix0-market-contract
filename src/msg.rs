use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::SellOffer;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Dummy {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Hello {},
}

// We define a custom struct for each query response
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

