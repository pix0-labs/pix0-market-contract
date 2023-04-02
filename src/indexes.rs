use crate::state::{ SellOffer, PurchaseHistory};
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const SELL_OFFERS_STORE : Map<(Addr,String), SellOffer> = Map::new("PIX0_SELL_OFFERS_STORE");

pub const PURCHASE_HISTORY_STORE : Map<(Addr,String), PurchaseHistory> = Map::new("PIX0_PHIST_STORE");

