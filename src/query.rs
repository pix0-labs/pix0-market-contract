use cosmwasm_std::{Deps, StdResult, Order, Addr };
use crate::indexes::sell_offers_store;
use crate::state::SellOffer;
use crate::msg::SellOffersWithParamsResponse;
use std::convert::TryInto;

pub const DEFAULT_LIMIT : u32 = 10;

pub const MAX_LIMIT : u32 = 20;

pub fn get_sell_offers_of(deps : Deps,
    owner : Addr, 
    status : Option<u8>, 
    start: Option<u32>, limit: Option<u32>) 
    ->StdResult<SellOffersWithParamsResponse> {    
   
    let offers : StdResult<Vec<SellOffer>> = 

    sell_offers_store()
    .idx.offers
    .prefix(owner)
    .range(deps.storage, None, None, Order::Ascending)
    .map(|offer| {
        
        let (_k, s) = offer?;
        Ok (
            SellOffer { 
                owner : s.owner,
                price : s.price,
                offer_id : s.offer_id,
                collection_info : s.collection_info,
                token_id : s.token_id,
                buy_offers : s.buy_offers,
                allowed_direct_buy : s.allowed_direct_buy,
                status : s.status,
                deal_close_type : s.deal_close_type,
                date_created : s.date_created,
                date_updated : s.date_updated,
            }
        )
    }).collect();


    if offers.is_err() {

        return Ok(SellOffersWithParamsResponse::empty_response())
    
    }

    let offers = offers.unwrap();

    let res : (Vec<SellOffer>,usize) = filter_sell_offer_result(offers, status, start, limit);

    Ok(SellOffersWithParamsResponse {
        offers: res.0,
        total : Some(res.1.try_into().unwrap_or(0)),
        start : start,
        limit : limit
    })
    
}


fn filter_sell_offer_result(offers : Vec<SellOffer>, 
    status : Option<u8>,
    start : Option<u32>,
    limit: Option<u32>) -> (Vec<SellOffer>,usize){

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let skip = start.unwrap_or(0) as usize ;
    
    let res = filter_sell_offer_result_all(offers, status);

    (res.clone()
    .into_iter()
    .skip(skip)
    .take(limit)
    .collect::<Vec<SellOffer>>(), res.len())
}

fn filter_sell_offer_result_all(offers : Vec<SellOffer>, 
    status : Option<u8>) -> Vec<SellOffer>{
   
    if  status.is_some() {

        offers.into_iter().filter(|c| 
        c.status == status.unwrap())
        .collect::<Vec<SellOffer>>()
    }
    else {

       offers      
    }

}



pub (crate) fn internal_get_sell_offer(deps: Deps, owner : Addr, token_id : String  ) -> Option<SellOffer>{

    let _key = (owner, token_id );

    let stored_sell_offer = sell_offers_store().key(_key);
    
    let res = stored_sell_offer.may_load(deps.storage);

    if res.is_ok() {

        let value = res.unwrap_or_else(|_| {
            None
        });

        value 
    }
    else {

        None
    }
}

#[allow(dead_code)]
pub (crate) fn internal_get_sell_offer_by_id(deps: Deps, offer_id : String   ) -> Option<SellOffer>{

    let res =  sell_offers_store().idx.offers_by_id.item(deps.storage, offer_id);
    
    if res.is_ok() {

        let value = res.unwrap_or_else(|_| {
            None
        });

        Some(value.unwrap().1)
    }
    else {

        None
    }
}
