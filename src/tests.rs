#[cfg(test)]
mod tests {
  
    use crate::state::*;
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr,  Coin, Uint128, };
    use crate::msg::*;
    use crate::contract::*;
    use pix0_contract_common::msg::InstantiateMsg;
    use pix0_contract_common::state::*;
   // use crate::ins::*;
    use crate::query::*;
   

    const DEFAULT_PRICE_DENOM : &str = "uconst";
   
    // cargo test test_create_sell_offers -- --show-output
    #[test]
    fn test_create_sell_offers(){

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        println!("Test.create.sell.offers!");

        let admin =  Addr::unchecked(owner.to_string());
        let admin2 =  Addr::unchecked("archway1upspu5660q39adv768z8ffk44ta6lzd4nfw2zw".to_string());
        let admin3 =  Addr::unchecked("archway1cz5a70ja86ak40de7r6vgm2lr9mtgvue5sj5kp".to_string());

        let ins = InstantiateMsg {

            allowed_admins : Some(vec![admin.clone()]),
            treasuries : Some(vec![admin,admin2, admin3]),
            contracts : None, 
            fees : Some(vec![ 
                Fee {name : "CREATE_SELL_OFFER_FEE".to_string(),
                value : Coin { amount : Uint128::from(3500u64), denom : "uconst".to_string()}},
            ]) ,
            log_last_payment : Some(true)

        };

        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), ins.clone());
       
        println!("Instantiated::{:?}\n", res);
       

        for x in 0..10 {
     
            let tid = format!("Tk_00{}", x);
            //let oid = format!("Offer_00{}", x);

            let price : Coin = Coin {
                amount : Uint128::from((3500 * (x+1)) as u64 ),
                denom : DEFAULT_PRICE_DENOM.to_string(), 
            };

            let s = SellOffer {
                token_id : tid, 
                owner : Addr::unchecked(owner), 
                collection_info : None,
                offer_id : None,//Some(oid),
                price : price, 
                status : 0,
                buy_offers : vec![],
                allowed_direct_buy : true, 
                deal_close_type : None,
                date_created : None,
                date_updated : None, 
            };

            let create_so = ExecuteMsg::CreateSellOffer {
                offer : s.clone()
            };
  
             let _res = execute(deps.as_mut(), mock_env(), info.clone(), 
             create_so.clone());
  
             if _res.is_err() {
  
                 println!("Error.creating so:{}, error:is::{:?}", s.token_id, _res);
             }
             /*
             else {

                println!("{}.Created.so:{:?}", x, _res);
             } */

        }

        let res = get_sell_offers_of(deps.as_ref(), Addr::unchecked(owner), None, None, None);

        for o in res.ok().unwrap().offers {

            println!("Offer.id::{:?}", o.offer_id);
        }

    
        let oid = String::from("OF4282D6668CBCC8EF");

        let o = internal_get_sell_offer_by_id(deps.as_ref(), oid.clone());

        assert_eq!(oid, o.unwrap().offer_id.unwrap());
        
    }

}