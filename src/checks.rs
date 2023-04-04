use cosmwasm_std::{Deps, MessageInfo, Addr, Uint128, Coin };
use crate::error::ContractError;
use crate::indexes::{sell_offers_store, BUY_OFFERS_STORE};
use crate::state::{SellOffer, SELL_STATUS_CLOSED};

pub (crate) fn sell_offer_exists_by_offer_id( deps: &Deps, offer_id : String ) -> Option<SellOffer> {
   
    let loaded_sell_offer = sell_offers_store()
    .idx.offers_by_id.item(deps.storage, offer_id);
    
    match loaded_sell_offer {

        Ok (s) => {
            
            if s.is_some() {

                Some(s.unwrap().1)
            }
            else {

                None 
            }
        },

        Err(_)=> None, 
    }

}


pub (crate) fn sell_offer_exists( deps: &Deps, info: MessageInfo, token_id : String ) -> Option<SellOffer> {

    let owner = info.clone().sender;
    
    let _key = (owner, token_id);

    let loaded_sell_offer = sell_offers_store()
    .idx.offers.item(deps.storage, _key);
    
    match loaded_sell_offer {

        Ok (c) => {
            if c.is_some() {
                Some(c.unwrap().1)
            }
            else {
                None
            }
        },

        Err(_)=> None, 
    }

}


pub (crate) fn check_sell_offer_exists (deps : &Deps,info: &MessageInfo, token_id : String, error_on_exists : bool ) 
-> Result<Option<SellOffer>, ContractError> {

    if error_on_exists {
        let so = sell_offer_exists(&deps, info.clone(), token_id.clone());
        
        if so.is_some() {

            return Err(ContractError::SellOfferAlreadyExists { 
                message: format!("SellOffer for {} already exists!",token_id).to_string() } );
      
        }
        Ok(None)
    }
    else {

        let so = sell_offer_exists(&deps, info.clone(), token_id.clone());

        if so.is_none() {

            return Err(ContractError::SellOfferNotFound { 
                message: format!("SellOffer for {} not found!",token_id).to_string() } );
      
        }
        Ok(so)
    }
   
}


pub (crate) fn sell_offer_exists_by (deps : &Deps, offer_id : String, error_on_exists : bool ) -> 
Result<Option<SellOffer>, ContractError> {

    if error_on_exists {

        let so = sell_offer_exists_by_offer_id(&deps, offer_id.clone());
        if so.is_some() {   

            return Err(ContractError::SellOfferAlreadyExists { 
                message: format!("SellOffer {} already exists!",offer_id).to_string() } );
      
        }
        Ok(None)
    }
    else {

        let so = sell_offer_exists_by_offer_id(&deps, offer_id.clone());
        if  so.is_none() {

            return Err(ContractError::SellOfferNotFound { 
                message: format!("SellOffer {} not found!",offer_id).to_string() } );
      
        }
        Ok(so)
    }
   
}

pub (crate) fn check_sell_offer_cancellable( deps: &Deps, info: MessageInfo, token_id : String ) -> Result<SellOffer, ContractError>{

    let owner = info.clone().sender;
    
    let _key = (owner, token_id.clone());

    let loaded_sell_offer = sell_offers_store()
    .idx.offers.item(deps.storage, _key);
    
    match loaded_sell_offer {

        Ok (c) => {
            if c.is_some() {

                let so = c.unwrap().1;
                if so.status == SELL_STATUS_CLOSED {
                    return Err(ContractError::SellOfferIsAlreadyClosed { 
                        message: format!("SellOffer for {} is already closed!",token_id).to_string() } );
                }
                else {

                    Ok(so)
                }
            }
            else {
                return Err(ContractError::SellOfferNotFound { 
                    message: format!("SellOffer for {} not found!",token_id).to_string() } );
            }
        },

        Err(_)=>   return Err(ContractError::SellOfferNotFound { 
            message: format!("SellOffer for {} not found!",token_id).to_string() } ), 
    }

    
}


pub (crate) fn check_buy_offer_exists (deps : Deps, owner : &Addr, sell_offer_id : String, exists_on_error : bool) -> Result<(), ContractError> {

    let _key = (sell_offer_id,owner.clone());

    let stored_bo = BUY_OFFERS_STORE.key(_key.clone());
    
    let bo_result = stored_bo.may_load(deps.storage);
    
    if exists_on_error {

        if bo_result.is_ok() {

            if bo_result.ok().unwrap().is_some() {
                return Err(ContractError::BuyOfferAlreadyExists { 
                    message: format!("Buy Offer for {:?} already exists!", owner) } );
            }
      
        }
        Ok(())
    
    }
    else {

        if bo_result.is_ok() {

            if bo_result.ok().unwrap().is_none() {
                return Err(ContractError::BuyOfferNotFound { 
                    message: format!("Buy Offer for {:?} NOT found!", owner) } );
          
            }
      
        }
        else 
        if bo_result.is_err() {
            return Err(ContractError::BuyOfferNotFound { 
                message: format!("Buy Offer for {:?} NOT found!", owner) } );
      
        }
        Ok(())
    }
   
}


pub (crate) fn check_is_fund_sufficient (info : MessageInfo, required_fund : Coin) -> Result<(), ContractError> {

    let sent_funds: Vec<Coin> = info.funds.clone();

    if required_fund.amount == Uint128::from(0u8) {
        return Err(ContractError::InvalidRequiredFund { 
            message: String::from("Required fund cannot be zero!")} 
        );
    }

    if sent_funds.len() == 0 {
        return Err(ContractError::InsufficientFund { 
            message: format!("Sent fund 0{} is less than required {}{}!",
            required_fund.denom, required_fund.amount, required_fund.denom) } );
    }

    let first_fund = sent_funds.get(0).unwrap();

    if first_fund.amount < Uint128::from(required_fund.amount) ||
    first_fund.denom != required_fund.denom {
        return Err(ContractError::InsufficientFund { 
            message: format!("Sent fund {}{} is less than required {}{}!",first_fund.amount,
        first_fund.denom, required_fund.amount, required_fund.denom) } );
    }
    else {
        Ok(())
    }
}

