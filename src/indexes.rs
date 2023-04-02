use crate::state::{BuyOffer,SellOffer, PurchaseHistory};
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const BUY_OFFERS_STORE : Map<(Addr,String), BuyOffer> = Map::new("PIX0_BUY_OFFERS_STORE");

pub const PURCHASE_HISTORY_STORE : Map<(Addr,String), PurchaseHistory> = Map::new("PIX0_PHIST_STORE");

