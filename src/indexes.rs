use crate::state::{ SellOffer, PurchaseHistory, BuyOffer, SimpleCollectionInfo};
use cosmwasm_std::Addr;
use cw_storage_plus::{UniqueIndex, Index, IndexList, IndexedMap, Map};
use crate::utils::to_unique_token_id;

pub const PURCHASE_HISTORY_STORE : Map<(Addr,String), PurchaseHistory> = Map::new("PIX0_PHIST_STORE");

pub const BUY_OFFERS_STORE : Map<(String, Addr), BuyOffer> = Map::new("PIX0_BUY_OFFERS_STORE");

pub const COLLECTION_INDEX : Map<String, SimpleCollectionInfo> = Map::new("PIX0_COLLECTION_INDEX");

pub struct SellOfferIndexes<'a> {

    pub offers : UniqueIndex<'a, (Addr,String), SellOffer>,

    pub offers_by_id : UniqueIndex<'a, String, SellOffer>,
}


impl IndexList<SellOffer> for SellOfferIndexes<'_> {

    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<SellOffer>> + '_> {

        let v : Vec<&dyn Index<SellOffer>> = vec![&self.offers, &self.offers_by_id];
        Box::new(v.into_iter())
    } 
}

pub fn sell_offers_store<'a>() -> IndexedMap<'a,(Addr,String), SellOffer, SellOfferIndexes<'a>> {

    let indexes = SellOfferIndexes {

        offers : UniqueIndex::new(|s| (s.owner.clone(),
        to_unique_token_id(s.contract_addr.clone(), 
        s.token_id.clone())), "SELL_OFFERS"),

        offers_by_id :  UniqueIndex::new(|s|  
        s.offer_id.clone().unwrap_or(String::from("unknown_id")), "SELL_OFFERS_BY_ID"),
    };

    IndexedMap::new("PIX0_SELL_OFFERS_STORE", indexes)
}



/* 
04/Apr/23 <ketyung@techchee.com>
MultiIndex doesn't seem to work accordingly, the last error known
was ParseErr { target_type: "pix0_market_contract::state::BuyOffer", msg: "missing field `owner`" }*/
/* 
pub struct BuyOfferIndexes<'a> {

    pub offers : UniqueIndex<'a, (Addr,String), BuyOffer>,

    pub sell_offers : MultiIndex<'a, String, BuyOffer, (Addr,String)>,
}



impl IndexList<BuyOffer> for BuyOfferIndexes<'_> {

    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<BuyOffer>> + '_> {

        let v : Vec<&dyn Index<BuyOffer>> = vec![&self.offers, &self.sell_offers];
        Box::new(v.into_iter())
    } 
}

pub fn buy_offers_store<'a>() -> IndexedMap<'a,(Addr,String), BuyOffer, BuyOfferIndexes<'a>> {

    let indexes = BuyOfferIndexes {

        offers : UniqueIndex::new(|s| (s.owner.clone(),
        s.sell_offer_id.clone()), "pk_BUY_OFFERS"),

        sell_offers : MultiIndex::new(|s: &BuyOffer| 
        s.sell_offer_id.clone(), 
        "pk_BUY_OFFERS",
        "BUY_OFFERS_SELL_OFFERS"),

    };

    IndexedMap::new("BUY_OFFERS_STORE", indexes)
}
*/
