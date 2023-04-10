use cosmwasm_std::{DepsMut, Deps, Env, Response, MessageInfo, Addr, Uint128, Coin, BankMsg, Attribute};
use crate::state::{SellOffer, SELL_STATUS_NEW, BuyOffer, SELL_STATUS_CLOSED, DEAL_CLOSED_OFFER_ACCEPTED,
DEAL_CLOSED_AT_DIRECT_BUY};
use crate::indexes::{sell_offers_store, BUY_OFFERS_STORE};
use crate::error::ContractError;
use crate::checks::*;
use crate::utils::to_unique_token_id;
use crate::query::{internal_get_buy_offer,internal_get_sell_offer_by_id, get_buy_offers_by};
use pix0_contract_common::state::{Contract,Fee};
use pix0_contract_common::funcs::{try_paying_contract_treasuries};
use pix0_market_handlers::triggers::{trigger_send_nft_to_contract, trigger_send_nft_from_contract};
use crate::collection_index::{save_collection_index, remove_sell_offer_from_index};
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

    check_sell_offer_exists (&deps.as_ref(), &info, offer.token_id.clone(), offer.contract_addr.clone(), true )?;

    let date_created = _env.block.time;

    let mut offer_id = offer.offer_id.clone();
    if offer_id.is_none() {
        offer_id = offer.to_offer_id();
    }
 
    let new_offer = SellOffer {
        owner : owner.clone(),
        contract_addr : offer.contract_addr.clone(),
        token_id : offer.token_id.clone(),
        offer_id : offer_id,
        price : offer.price,
        collection_info : offer.collection_info.clone(),
        allowed_direct_buy : offer.allowed_direct_buy,
        status : SELL_STATUS_NEW,
        deal_close_type : None, 
        date_created : Some(date_created),
        date_updated : Some(date_created),
    };

    let bmsgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info.clone(), "CREATE_SELL_OFFER_FEE")?;
 

    let _key = (owner, to_unique_token_id(offer.contract_addr.clone(), offer.token_id.clone()) );

    sell_offers_store().save(deps.storage, _key, &new_offer)?;

    save_collection_index(deps, offer.collection_info);

    // transfer NFT from the seller to the contract
    let cmsg = trigger_send_nft_to_contract(info, offer.token_id, offer.contract_addr)?;

    Ok(Response::new()
    .add_messages(bmsgs)
    .add_attribute("action", "create-sell-offer")
    .add_message(cmsg))
    

}


pub fn update_sell_offer(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    sell_offer : SellOffer) -> Result<Response, ContractError> {

    let owner = info.clone().sender;

    let so = check_sell_offer_exists (&deps.as_ref(), &info, sell_offer.token_id.clone(), 
    sell_offer.contract_addr.clone(), false)?;

    let mut offer_to_update = so.unwrap();

    offer_to_update.price = sell_offer.price;
    offer_to_update.allowed_direct_buy = sell_offer.allowed_direct_buy;

    if sell_offer.collection_info.is_some() {
        offer_to_update.collection_info = sell_offer.collection_info;
    }

    offer_to_update.date_updated = Some(_env.block.time);

    let _key = (owner, 
        to_unique_token_id(sell_offer.contract_addr, sell_offer.token_id));

    sell_offers_store().save(deps.storage, _key.clone(), &offer_to_update)?;
    
    Ok(Response::new()
    .add_attribute("action", "update-sell-offer"))
  
}



pub fn cancel_sell_offer (
    mut deps: DepsMut ,  
    env : Env,
    info: MessageInfo,
    token_id : String, 
    contract_addr : String) -> Result<Response, ContractError> {
    
    let so = check_sell_offer_cancellable (&deps.as_ref(), info, token_id.clone(), contract_addr.clone())?;

    // refund all buy offers first before cancelling them!
    let resp = refund_all_buy_offers(deps.as_ref(), so.offer_id.clone().unwrap())
    .add_attribute("action", "cancel-sell-offer");

    cancel_all_buy_offers(deps.branch(), so.offer_id.unwrap(), None);

    let _key = (so.owner.clone(), to_unique_token_id(contract_addr, token_id) );
    sell_offers_store().remove(deps.branch().storage, _key.clone())?;

    remove_sell_offer_from_index(deps, so.collection_info);

    // transfer token back to seller (owner of sell offer)
    let cmsg = trigger_send_nft_from_contract(env, so.token_id, so.owner.clone().to_string(), so.contract_addr)?;

    Ok(resp.add_message(cmsg))  

}

fn cancel_all_buy_offers(deps : DepsMut, sell_offer_id : String, except : Option<BuyOffer>)  {
  
    let buy_offers_res = 
    get_buy_offers_by(deps.as_ref(),  sell_offer_id.clone(), None, None, None);

    if buy_offers_res.is_ok() {

        let mut buy_offers : Vec<BuyOffer> = buy_offers_res.ok().unwrap().offers;

        if except.is_some() {
            buy_offers.retain(|b| b.owner != except.clone().unwrap().owner);
        }

        for b in buy_offers.iter() {

            let _key = (sell_offer_id.clone(),b.owner.clone());

            BUY_OFFERS_STORE.remove(deps.storage, _key.clone());

        }
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

#[allow(dead_code)]
fn pay_so_owner_and_royalties (price : Coin, sell_offer : SellOffer) -> Response {

    let collection_info = sell_offer.collection_info;
    let cinfo = collection_info.clone();

    if cinfo.is_some() && cinfo.unwrap().royalties.is_some(){

        let royalties = collection_info.unwrap().royalties.unwrap();
        

        let mut total_royalty_percentage : u8= 0;

        let mut bmsgs : Vec<BankMsg> = vec![];

        let mut attribs : Vec<Attribute> = vec![];


        for r in royalties {

            let amount = Coin { amount : (Uint128::from(r.percentage)/Uint128::from(100u8)) * price.amount, 
                denom : price.denom.clone()};

            bmsgs.push( BankMsg::Send {
                    to_address: r.wallet.clone().into(),
                    amount: vec![amount],
            });

            attribs.push (Attribute{ key : String::from("to"), value : r.wallet.to_string()});

            total_royalty_percentage += r.percentage ;

        }

        let remaining_percentage = 100 - total_royalty_percentage;
        let amount = Coin {amount : (Uint128::from(remaining_percentage)/Uint128::from(100u8)) * price.amount,
            denom : price.denom} ;

        Response::new().add_message(BankMsg::Send {
            to_address: sell_offer.owner.clone().into(),
            amount: vec![amount],
        })
        .add_attribute("action", "accept-buy-offer")
        .add_attribute("to", sell_offer.owner)
        .add_messages(bmsgs)
        .add_attributes(attribs)
        
    }
    else {
        internal_transfer_from_escrow(sell_offer.owner, price,
        "accept-buy-offer")

    }

}

fn accept_bo_and_refund_others(deps : Deps, buy_offer : BuyOffer, sell_offer : SellOffer) -> Response {

    let res = pay_so_owner_and_royalties(buy_offer.price.clone(), sell_offer.clone());
    
    let mesgs = refund_buy_offers(deps,  sell_offer.offer_id.unwrap(), Some(buy_offer));

    res.clone()
    .add_messages( mesgs.0)
    .add_attributes(mesgs.1);

    res 
}

fn refund_all_buy_offers(deps : Deps,  sell_offer_id : String) -> Response{

    let mesgs = refund_buy_offers(deps, sell_offer_id, None);

    Response::new()
    .add_messages(mesgs.0)
    .add_attributes(mesgs.1)
}

fn refund_buy_offers(deps : Deps,  sell_offer_id : String, except : Option<BuyOffer>) -> (Vec<BankMsg>, Vec<Attribute>) {

    let mut mesgs : Vec<BankMsg> = vec![];
    let mut attrbs : Vec<Attribute> = vec![];

    let buy_offers_res = 
    get_buy_offers_by(deps,  sell_offer_id, None, None, None);

    if buy_offers_res.is_ok() {

        let mut buy_offers : Vec<BuyOffer> = buy_offers_res.ok().unwrap().offers;

        if except.is_some() {
            buy_offers.retain(|b| b.owner != except.clone().unwrap().owner);
        }

        for b in buy_offers.iter() {
                      
            mesgs.push(BankMsg::Send {
                to_address: b.owner.clone().into(),
                amount : vec![b.price.clone()],
            });
        
            attrbs.push(Attribute { key: String::from("to"), value: b.owner.clone().into() });

        }
    }

    (mesgs, attrbs)
   
} 


fn get_buy_offer_checked (deps: Deps, owner : Addr, sell_offer_id : String) -> Result<BuyOffer, ContractError>{

    let buy_offer = internal_get_buy_offer(deps, owner, sell_offer_id)?;

    if buy_offer.accepted {
        return Err(ContractError::BuyOfferAlreadyAccepted { message:
            format!("Buy offer {:?} already accepted!",buy_offer.owner) } );
    }

    Ok(buy_offer)
}

pub fn accept_buy_offer(mut deps: DepsMut, 
    _env : Env, info: MessageInfo,
    buy_offer_by : Addr, 
    sell_offer_id : String )  -> Result<Response, ContractError> {
    
    let mut bo = get_buy_offer_checked(deps.as_ref(), buy_offer_by.clone(), 
    sell_offer_id.clone())?;

    bo.accepted = true ;
    bo.date_updated = Some(_env.block.time);

    let mut so = internal_get_sell_offer_by_id(deps.as_ref(), sell_offer_id.clone())?;
    
    assert_eq!(info.sender, so.owner);

    let _key = (sell_offer_id.clone(), buy_offer_by);

    BUY_OFFERS_STORE.save(deps.storage, _key.clone(), &bo)?;

    so.status = SELL_STATUS_CLOSED;
    so.deal_close_type = Some(DEAL_CLOSED_OFFER_ACCEPTED);
    so.date_updated = Some(_env.block.time);

    let _key = (so.owner.clone(), so.token_id.clone());
    sell_offers_store().save(deps.storage, _key.clone(), &so)?;

    // remove the sell offer from collection index
    remove_sell_offer_from_index(deps.branch(), so.collection_info.clone());

    // trigger transfer the locked NFT in contract to buyer
    let cmsg = trigger_send_nft_from_contract(_env, so.token_id.clone(), bo.owner.clone().to_string(), so.contract_addr.clone())?;

    Ok(accept_bo_and_refund_others(deps.as_ref(),bo, so)
    .add_message(cmsg))
} 




fn direct_buy_and_refund_others(deps : Deps, price :Coin , sell_offer : SellOffer) -> Response {

    let res = pay_so_owner_and_royalties(price, sell_offer.clone());
    
    let mesgs = refund_buy_offers(deps, sell_offer.offer_id.unwrap(), None);

    res.clone()
    .add_messages( mesgs.0)
    .add_attributes(mesgs.1);

    res 
}

pub fn direct_buy(mut deps: DepsMut, 
    _env : Env, info: MessageInfo, 
    sell_offer_id : String )  -> Result<Response, ContractError> {

    let mut so = sell_offer_exists_by (&deps.as_ref(),sell_offer_id.clone(), false)?.unwrap();
  
    let price = so.price.clone();

    check_is_fund_sufficient(info.clone(), price.clone())?;

    so.status = SELL_STATUS_CLOSED;
    so.deal_close_type = Some(DEAL_CLOSED_AT_DIRECT_BUY);
    so.date_updated = Some(_env.block.time);

    let _key = (so.owner.clone(), so.token_id.clone());
    sell_offers_store().save(deps.storage, _key.clone(), &so)?;

     // remove the sell offer from collection index
     remove_sell_offer_from_index(deps.branch(), so.collection_info.clone());


     // trigger transfer the locked NFT in contract to direct buyer
    let cmsg = trigger_send_nft_from_contract(_env, so.token_id.clone(), 
    info.sender.to_string(), so.contract_addr.clone())?;

    Ok(direct_buy_and_refund_others(deps.as_ref(), price, so).add_message(cmsg))

} 
 
