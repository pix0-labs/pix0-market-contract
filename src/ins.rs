use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, Addr};
use crate::state::{SellOffer, SELL_STATUS_NEW};
use crate::indexes::sell_offers_store;
use crate::error::ContractError;
use crate::query::internal_get_sell_offer;
use pix0_contract_common::state::{Contract,Fee};
use pix0_contract_common::funcs::{try_paying_contract_treasuries};

/*
Wrapper function
*/
pub fn update_contract_info (deps: DepsMut, 
    _env : Env, info: MessageInfo,
    _fees : Option<Vec<Fee>>, treasuries : Option<Vec<Addr>>, 
    contracts : Option<Vec<Contract>>, 
    _log_last_payment : Option<bool>, 
 ) -> Result<Response, ContractError> {

    let res =  pix0_contract_common::funcs::update_contract_info(
        deps, _env, info, _fees, treasuries, contracts, _log_last_payment);
           
    match res {

        Ok(r)=> Ok(r),

        Err(e)=> Err(ContractError::from(e)),
    }
}


pub fn sell_offer_exists( deps: &DepsMut, info: MessageInfo, token_id : String ) -> bool {

    let owner = info.clone().sender;
    
    let _key = (owner, token_id);

    let loaded_sell_offer = sell_offers_store()
    .idx.offers.item(deps.storage, _key);
    
    let mut exists = false; 

    match loaded_sell_offer {

        Ok (c) => {
            if c.is_some() {
                exists = true
            }
        },

        Err(_)=> exists = false, 
    }

    return exists;
}



pub fn create_sell_offer(mut deps: DepsMut, 
_env : Env, info: MessageInfo, offer : SellOffer)  -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    if sell_offer_exists(&deps, info.clone(), offer.token_id.clone()) {

        return Err(ContractError::SellOfferAlreadyExists { 
            message: format!("SellOffer for {} already exists!", offer.token_id.clone()).to_string() } );
  
    }

    let date_created = _env.block.time;

    let mut offer_id = offer.offer_id.clone();
    if offer_id.is_none() {
        offer_id = offer.to_offer_id();
    }
 
    let new_offer = SellOffer {
        owner : owner.clone(),
        token_id : offer.token_id.clone(),
        offer_id : offer_id,
        buy_offers : vec![],
        price : offer.price,
        collection_info : offer.collection_info,
        allowed_direct_buy : offer.allowed_direct_buy,
        status : SELL_STATUS_NEW,
        deal_close_type : None, 
        date_created : Some(date_created),
        date_updated : Some(date_created),
    };

    let bmsgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info, "CREATE_BUY_OFFER_FEE")?;
 

    let _key = (owner, offer.token_id );

    sell_offers_store().save(deps.storage, _key.clone(), &new_offer)?;


    Ok(Response::new()
    .add_messages(bmsgs)
    .add_attribute("method", "create-sell-offer"))
    

}


pub fn update_sell_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    sell_offer : SellOffer) -> Result<Response, ContractError> {

    let owner = info.clone().sender;


    let offer = internal_get_sell_offer(deps.as_ref(), owner.clone(), sell_offer.token_id.clone());

    if offer.is_none () {

        return Err(ContractError::SellOfferNotFound { 
            message: format!("SellOffer for {} not found!", sell_offer.token_id.clone()).to_string() } );
  
    }

    let mut offer_to_update = offer.unwrap();

    
    offer_to_update.price = sell_offer.price;
    offer_to_update.allowed_direct_buy = sell_offer.allowed_direct_buy;

    if sell_offer.collection_info.is_some() {
        offer_to_update.collection_info = sell_offer.collection_info;
    }

    offer_to_update.date_updated = Some(_env.block.time);

    let _key = (owner, sell_offer.token_id);

    sell_offers_store().save(deps.storage, _key.clone(), &offer_to_update)?;
    
    Ok(Response::new()
    .add_attribute("method", "update-sell-offer"))
  
}

/*
pub fn create_buy_offer(mut deps: DepsMut, 
    _env : Env, info: MessageInfo, offer : SellOfferz)  -> Result<Response, ContractError> {

} */

