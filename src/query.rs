use cosmwasm_std::{Deps, StdResult, Order, Addr, Env, Coin };
use crate::indexes::{sell_offers_store, buy_offers_store};
use crate::state::{SellOffer, BuyOffer};
use crate::error::ContractError;
use crate::msg::{SellOffersWithParamsResponse, SellOfferResponse, BalanceResponse};
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


pub fn get_sell_offer_by_id(deps: Deps, offer_id : String ) -> StdResult<SellOfferResponse>{

    let offer = internal_get_sell_offer_by_id(deps,offer_id);

    if offer.is_ok() {
        Ok( SellOfferResponse {
            offer : Some(offer.ok().unwrap())
        })
    }
    else {
        Ok( SellOfferResponse {
            offer : None 
        })
    }
   
} 

pub (crate) fn internal_get_sell_offer(deps: Deps, owner : Addr, token_id : String  ) -> Result<SellOffer, ContractError>{

    let _key = (owner, token_id.clone() );

    let stored_sell_offer = sell_offers_store().key(_key);
    
    let res = stored_sell_offer.may_load(deps.storage);

    if res.is_ok() {

        let v = res.ok();
        if v.is_some() {

            return Ok(v.unwrap().unwrap());
        }
        else {
            return Err(ContractError::SellOfferNotFound { 
                message: format!("Sell Offer for {} not found!", token_id).to_string() } );
        }
    }
    else {

        return Err(ContractError::SellOfferNotFound { 
            message: format!("Sell Offer for {} not found!", token_id).to_string() } );
    }
}

#[allow(dead_code)]
pub (crate) fn internal_get_sell_offer_by_id(deps: Deps, offer_id : String   ) -> Result<SellOffer, ContractError>{

    let res =  sell_offers_store().idx.offers_by_id.item(deps.storage, offer_id.clone());
    
    if res.is_ok() {

        let v = res.ok();
        if v.is_some() {
            return Ok(v.unwrap().unwrap().1);
        }
        else {
            return Err(ContractError::SellOfferNotFound { 
                message: format!("Sell Offer {} not found!", offer_id).to_string() } );
        }
    }
    else {

        return Err(ContractError::SellOfferNotFound { 
            message: format!("Sell Offer {} not found!", offer_id).to_string() } );
    }
}

#[allow(dead_code)]
pub (crate) fn internal_get_buy_offer(deps: Deps, owner : Addr, sell_offer_id : String   ) -> Result<BuyOffer, ContractError>{

    let _key = (owner.clone(),sell_offer_id.clone());

    let stored_bo = buy_offers_store().key(_key.clone());
    
    let res = stored_bo.may_load(deps.storage);
    
    if res.is_ok() {

        let v = res.ok();
        if v.is_some() {
            return Ok(v.unwrap().unwrap());
        }
        else {
            return Err(ContractError::BuyOfferNotFound { 
                message: format!("Buy Offer {} not found!", sell_offer_id).to_string() } );
        }
    }
    else {

        return Err(ContractError::BuyOfferNotFound { 
            message: format!("Buy Offer {} not found!", sell_offer_id).to_string() } );
    }
}


pub (crate) fn internal_get_balance_of_escrow(deps: Deps, env : Env, denom : impl Into<String> ) ->Option<Coin> {

    let balance = deps.querier.query_balance(&env.contract.address, denom);

    if balance.is_ok() {
        balance.ok()
    }
    else {

        None 
    }
}


pub fn get_balance_of_escrow(deps: Deps, env : Env, denom : impl Into<String> ) -> StdResult<BalanceResponse> {

   
    Ok(BalanceResponse{
        amount : internal_get_balance_of_escrow(deps, env, denom)
    })
}