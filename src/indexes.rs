use crate::state::{ SellOffer, PurchaseHistory};
use cosmwasm_std::Addr;
use cw_storage_plus::{UniqueIndex, Index, IndexList, IndexedMap, Map};

pub const PURCHASE_HISTORY_STORE : Map<(Addr,String), PurchaseHistory> = Map::new("PIX0_PHIST_STORE");

pub struct SellOfferIndexes<'a> {

    // unique index by wallet address
    pub offers : UniqueIndex<'a, (Addr,String), SellOffer>,

    // unique index by name and symbols
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
        s.token_id.clone()), "SELL_OFFERS"),

        offers_by_id :  UniqueIndex::new(|s|  
        s.offer_id.clone().unwrap_or(String::from("unknown_id")), "SELL_OFFERS_BY_ID"),
    };

    IndexedMap::new("SELL_OFFERS_STORE", indexes)
}

