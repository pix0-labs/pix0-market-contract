use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, Addr};
use crate::state::{SellOffer, SELL_STATUS_NEW};
use crate::indexes::SELL_OFFERS_STORE;
use crate::error::ContractError;
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


pub fn create_sell_offer(mut deps: DepsMut, 
_env : Env, info: MessageInfo, offer : SellOffer)  -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    let date_created = _env.block.time;
 
    let new_offer = SellOffer {

        owner : owner.clone(),
        token_id : offer.token_id.clone(),
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

    SELL_OFFERS_STORE.save(deps.storage, _key.clone(), &new_offer)?;


    Ok(Response::new()
    .add_messages(bmsgs)
    .add_attribute("method", "create-sell-offer"))
    

}

