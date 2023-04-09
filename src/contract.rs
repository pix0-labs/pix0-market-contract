#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::ins::{create_sell_offer,update_contract_info, create_buy_offer, 
update_buy_offer, cancel_buy_offer, update_sell_offer, cancel_sell_offer, 
transfer_to_escrow, accept_buy_offer, direct_buy};
use crate::query::{get_sell_offers_of, get_sell_offer_by_id, get_balance_of_escrow,
get_buy_offers_by, get_buy_offers_of, get_sell_offers, get_collection_indexes, get_collection_index};
use pix0_contract_common::msg::InstantiateMsg;
use pix0_contract_common::funcs::create_contract_info;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pix0-market-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
        
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    create_contract_info(deps, _env, info.clone() ,_msg.allowed_admins,
    _msg.treasuries, _msg.fees, _msg.contracts,_msg.log_last_payment)?;
  

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        
        ExecuteMsg::UpdateContractInfo { fees, treasuries , contracts,  log_last_payment} =>
        update_contract_info(deps, _env, _info, fees, treasuries, contracts,log_last_payment),

        ExecuteMsg::CreateSellOffer { offer } => create_sell_offer(deps, _env,_info, offer),

        ExecuteMsg::UpdateSellOffer { offer } => update_sell_offer(deps, _env,_info, offer),
      
        ExecuteMsg::CancelSellOffer { token_id , contract_addr} => 
        cancel_sell_offer(deps, _env, _info, token_id, contract_addr),

        ExecuteMsg::CreateBuyOffer { buy_offer, sell_offer_id } => 
        create_buy_offer(deps, _env,_info, buy_offer, sell_offer_id),

        ExecuteMsg::UpdateBuyOffer { buy_offer, sell_offer_id } => 
        update_buy_offer(deps, _env,_info, buy_offer, sell_offer_id),

        ExecuteMsg::CancelBuyOffer { sell_offer_id } => 
        cancel_buy_offer(deps, _env,_info, sell_offer_id),

        ExecuteMsg::AcceptBuyOffer { buy_offer_by, sell_offer_id } => 
        accept_buy_offer(deps, _env,_info, buy_offer_by, sell_offer_id),

        ExecuteMsg::DirectBuy {sell_offer_id} =>
        direct_buy(deps, _env, _info, sell_offer_id),

        ExecuteMsg::TestTransferToEscrow { coin } => 
        transfer_to_escrow(_env,coin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSellOffers {
            status, collection_info, start, limit
        } => to_binary(&get_sell_offers(_deps, status, collection_info, start , limit)?),

        QueryMsg::GetSellOffersOf {
            owner, status, start, limit
        } => to_binary(&get_sell_offers_of(_deps, owner, status,start , limit)?),

        QueryMsg::GetSellOfferById {
            offer_id
        } => to_binary(&get_sell_offer_by_id(_deps, offer_id)?),

        QueryMsg::GetBalanceOfEscrow {
            denom 
        }  => to_binary(&get_balance_of_escrow(_deps, _env, denom)?),

        QueryMsg::GetBuyOffersOf {
            owner, accepted, start, limit  
        }  => to_binary(&get_buy_offers_of(_deps, owner, accepted, start, limit)?),

        QueryMsg::GetBuyOffersBy {
            sell_offer_id, accepted, start, limit  
        }  => to_binary(&get_buy_offers_by(_deps, sell_offer_id, accepted, start, limit)?),

        QueryMsg::GetCollectionIndexes { category, start, limit }=>
        to_binary(&get_collection_indexes(_deps, category, start, limit)?),

        QueryMsg::GetCollectionIndex { id } =>
        to_binary(&get_collection_index(_deps, id)?),


    }
}
