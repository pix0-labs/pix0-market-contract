use cosmwasm_std::{DepsMut, Deps, Env, Response, MessageInfo, Addr, Uint128, Coin, BankMsg};
use crate::state::{SellOffer, SELL_STATUS_NEW, BuyOffer};
use crate::indexes::{sell_offers_store, BUY_OFFERS_STORE};
use crate::error::ContractError;
use crate::query::{internal_get_sell_offer, internal_get_buy_offer,internal_get_sell_offer_by_id, get_buy_offers_by};
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



pub fn sell_offer_exists_by_offer_id( deps: &Deps, offer_id : String ) -> bool {

   
    let loaded_sell_offer = sell_offers_store()
    .idx.offers_by_id.item(deps.storage, offer_id);
    
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


pub fn sell_offer_exists( deps: &Deps, info: MessageInfo, token_id : String ) -> bool {

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


fn check_sell_offer_exists (deps : &Deps,info: &MessageInfo, token_id : String, error_on_exists : bool ) -> Result<(), ContractError> {

    if error_on_exists {
        if sell_offer_exists(&deps, info.clone(), token_id.clone()) {

            return Err(ContractError::SellOfferAlreadyExists { 
                message: format!("SellOffer for {} already exists!",token_id).to_string() } );
      
        }
        Ok(())
    }
    else {

        if !sell_offer_exists(&deps, info.clone(), token_id.clone()) {

            return Err(ContractError::SellOfferNotFound { 
                message: format!("SellOffer for {} not found!",token_id).to_string() } );
      
        }
        Ok(())
    }
   
}


fn sell_offer_exists_by (deps : &Deps, offer_id : String, error_on_exists : bool ) -> Result<(), ContractError> {

    if error_on_exists {
        if sell_offer_exists_by_offer_id(&deps, offer_id.clone()) {

            return Err(ContractError::SellOfferAlreadyExists { 
                message: format!("SellOffer {} already exists!",offer_id).to_string() } );
      
        }
        Ok(())
    }
    else {

        if !sell_offer_exists_by_offer_id(&deps, offer_id.clone()) {

            return Err(ContractError::SellOfferNotFound { 
                message: format!("SellOffer {} not found!",offer_id).to_string() } );
      
        }
        Ok(())
    }
   
}


pub fn create_sell_offer(mut deps: DepsMut, 
_env : Env, info: MessageInfo, offer : SellOffer)  -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    check_sell_offer_exists (&deps.as_ref(), &info, offer.token_id.clone(), true )?;

    let date_created = _env.block.time;

    let mut offer_id = offer.offer_id.clone();
    if offer_id.is_none() {
        offer_id = offer.to_offer_id();
    }
 
    let new_offer = SellOffer {
        owner : owner.clone(),
        token_id : offer.token_id.clone(),
        offer_id : offer_id,
        price : offer.price,
        collection_info : offer.collection_info,
        allowed_direct_buy : offer.allowed_direct_buy,
        status : SELL_STATUS_NEW,
        deal_close_type : None, 
        date_created : Some(date_created),
        date_updated : Some(date_created),
    };

    let bmsgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info, "CREATE_SELL_OFFER_FEE")?;
 

    let _key = (owner, offer.token_id );

    sell_offers_store().save(deps.storage, _key.clone(), &new_offer)?;


    Ok(Response::new()
    .add_messages(bmsgs)
    .add_attribute("action", "create-sell-offer"))
    

}


pub fn update_sell_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    sell_offer : SellOffer) -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    check_sell_offer_exists (&deps.as_ref(), &info, sell_offer.token_id.clone(), false)?;

    let mut offer_to_update = internal_get_sell_offer(deps.as_ref(), owner.clone(), sell_offer.token_id.clone())?;
    
    offer_to_update.price = sell_offer.price;
    offer_to_update.allowed_direct_buy = sell_offer.allowed_direct_buy;

    if sell_offer.collection_info.is_some() {
        offer_to_update.collection_info = sell_offer.collection_info;
    }

    offer_to_update.date_updated = Some(_env.block.time);

    let _key = (owner, sell_offer.token_id);

    sell_offers_store().save(deps.storage, _key.clone(), &offer_to_update)?;
    
    Ok(Response::new()
    .add_attribute("action", "update-sell-offer"))
  
}



pub fn remove_sell_offer (
    mut deps: DepsMut ,  
    info: MessageInfo,
    token_id : String) -> Result<Response, ContractError> {
    
    let owner = info.clone().sender;
    
    check_sell_offer_exists (&deps.as_ref(), &info, token_id.clone(), false)?;

    let _key = (owner.clone(), token_id );

    sell_offers_store().remove(deps.branch().storage, _key.clone())?;

    Ok(Response::new()
    .add_attribute("action", "remove-sell-offer"))  

}


fn check_buy_offer_exists (deps : Deps, owner : &Addr, sell_offer_id : String, exists_on_error : bool) -> Result<(), ContractError> {

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


fn check_is_fund_sufficient (info : MessageInfo, required_fund : Coin) -> Result<(), ContractError> {

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


// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
    .add_message(BankMsg::Send {
        to_address: to_address.clone().into(),
        amount,
    })
    .add_attribute("action", action)
    .add_attribute("to", to_address)
}


fn internal_transfer_to_escrow (env : Env, coin : Coin, action : &str ) -> Response {

    send_tokens(env.contract.address, vec![coin], action)
}

pub fn transfer_to_escrow(env : Env, coin : Coin) -> Result<Response, ContractError> {

    Ok(internal_transfer_to_escrow(env, coin, "transfer-to-escrow"))
}

fn refund_or_top_up (env : Env, amount : Uint128, denom : String, 
    recipient : Option<Addr>, action : &str) -> Response{

    let coin = Coin {
        amount : amount,
        denom : denom
    };

    if recipient.is_some() {
        internal_transfer_from_escrow(recipient.unwrap(), coin, action)
    }
    else {
        internal_transfer_to_escrow(env, coin, action )
    }
}




pub fn create_buy_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo, 
    buy_offer : BuyOffer, 
    sell_offer_id : String )  -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    internal_create_buy_offer(deps, _env, info, owner, buy_offer, sell_offer_id)
}

pub (crate) fn internal_create_buy_offer(mut deps: DepsMut, 
    _env : Env, info: MessageInfo, 
    owner : Addr, 
    buy_offer : BuyOffer, 
    sell_offer_id : String )  -> Result<Response, ContractError> {

    sell_offer_exists_by (&deps.as_ref(),sell_offer_id.clone(), false)?;

    let mut buy_offer  = buy_offer;
    buy_offer.owner = owner.clone();
    buy_offer.sell_offer_id = sell_offer_id.clone();
    buy_offer.date_created = Some(_env.block.time);
    buy_offer.date_updated = buy_offer.date_created;
    buy_offer.accepted = false ;

    check_buy_offer_exists( deps.as_ref(), &owner, sell_offer_id.clone(), true)?;

    let price = buy_offer.price.clone();

    check_is_fund_sufficient(info.clone(), price.clone())?;

    let bmsgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info, "CREATE_BUY_OFFER_FEE")?;
 
    let _key = (sell_offer_id,owner);

    BUY_OFFERS_STORE.save(deps.storage, _key.clone(), &buy_offer)?;
   
    Ok(internal_transfer_to_escrow(_env, price, "create-buy-offer")
    .add_messages(bmsgs))
} 

pub fn update_buy_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    buy_offer : BuyOffer, 
    sell_offer_id : String )  -> Result<Response, ContractError> {

    let owner = info.sender;

    let mut bo = internal_get_buy_offer(deps.as_ref(), owner.clone(), sell_offer_id.clone())?;

    let mut amt_diff : Uint128 = Uint128::from(0u8);
    let mut recipient : Option<Addr> = None; 

    if buy_offer.price.amount > bo.price.amount {

        amt_diff = buy_offer.price.amount - bo.price.amount;
    }
    else if buy_offer.price.amount < bo.price.amount {

        amt_diff = bo.price.amount - buy_offer.price.amount;
        recipient = Some(owner.clone());
    }

    bo.price = buy_offer.price.clone();
    bo.date_updated = Some(_env.block.time);


    let _key = (sell_offer_id, owner);

    BUY_OFFERS_STORE.save(deps.storage, _key.clone(), &bo)?;
   
    Ok(refund_or_top_up(_env, amt_diff,
        buy_offer.price.denom, recipient, "update-buy-offer"))
} 


fn refund_buy_offer(buy_offer : &BuyOffer, _env : Env, owner : Addr, action : &str)  -> Result<Response, ContractError>{

    let price = buy_offer.price.clone();
    
    Ok (refund_or_top_up(_env, price.amount, 
    price.denom, Some(owner), action))
}


pub fn cancel_buy_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo, 
    sell_offer_id : String )  -> Result<Response, ContractError> {

    let owner = info.sender;

    let bo = internal_get_buy_offer(deps.as_ref(), owner.clone(), sell_offer_id.clone())?;

    let _key = (sell_offer_id,owner.clone());

    BUY_OFFERS_STORE.remove(deps.storage, _key.clone());
   
    let res = refund_buy_offer(&bo, _env,owner, "cancel-buy-offer")?;   
    Ok(res)
   
} 


fn internal_transfer_from_escrow(recipient : Addr, coin : Coin, action : &str) -> Response {

    send_tokens(recipient, vec![coin],action)
}

fn accept_bo_and_refund_others(deps : Deps, so_owner : Addr, buy_offer : BuyOffer, sell_offer_id : String) -> Response {

    let res = internal_transfer_from_escrow(so_owner, buy_offer.price.clone(),
        "accept-buy-offer");
    
    let bmesgs = refund_other_buy_offers(deps, buy_offer, sell_offer_id);

    res.clone().add_messages( bmesgs);

    res 
}

fn refund_other_buy_offers(deps : Deps, except : BuyOffer, sell_offer_id : String) -> Vec<BankMsg> {

    let mut mesgs : Vec<BankMsg> = vec![];

    let buy_offers_res = 
    get_buy_offers_by(deps,  sell_offer_id, None, None, None);

    if buy_offers_res.is_ok() {

        let buy_offers : Vec<BuyOffer> = buy_offers_res.ok().unwrap().offers;
        for b in buy_offers.iter() {
            
            if b.owner != except.owner {

                mesgs.push(BankMsg::Send {
                    to_address: b.owner.clone().into(),
                    amount : vec![b.price.clone()],
                });
            }
        }
    }

    mesgs
   
} 


fn get_buy_offer_checked (deps: Deps, owner : Addr, sell_offer_id : String) -> Result<BuyOffer, ContractError>{

    let buy_offer = internal_get_buy_offer(deps, owner, sell_offer_id)?;

    if buy_offer.accepted {
        return Err(ContractError::BuyOfferAlreadyAccepted { message:
            format!("Buy offer {:?} already accepted!",buy_offer.owner) } );
    }

    Ok(buy_offer)
}

pub fn accept_buy_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    buy_offer_by : Addr, 
    sell_offer_id : String )  -> Result<Response, ContractError> {
    
    let owner = info.sender;

    let mut bo = get_buy_offer_checked(deps.as_ref(), buy_offer_by.clone(), 
    sell_offer_id.clone())?;

    bo.accepted = true ;
    bo.date_updated = Some(_env.block.time);

    let so = internal_get_sell_offer_by_id(deps.as_ref(), sell_offer_id.clone())?;
    
    assert_eq!(owner, so.owner);

    let _key = (sell_offer_id.clone(), buy_offer_by);

    BUY_OFFERS_STORE.save(deps.storage, _key.clone(), &bo)?;
   
    Ok(accept_bo_and_refund_others(deps.as_ref(), so.owner, bo, sell_offer_id))
} 


