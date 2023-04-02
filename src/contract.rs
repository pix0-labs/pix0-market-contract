#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::ins::{create_sell_offer,update_contract_info, create_buy_offer};
use crate::query::{get_sell_offers_of, get_sell_offer_by_id};
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

        ExecuteMsg::CreateBuyOffer { buy_offer, sell_offer_id } => 
        create_buy_offer(deps, _env,_info, buy_offer, sell_offer_id),


    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSellOffersOf {
            owner, status, start, limit
        } => to_binary(&get_sell_offers_of(_deps, owner, status, start , limit)?),

        QueryMsg::GetSellOfferById {
            offer_id
        } => to_binary(&get_sell_offer_by_id(_deps, offer_id)?),

    }
}
