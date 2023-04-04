#[cfg(test)]
mod tests {
  
    use crate::state::*;
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr,  Coin, Uint128, DepsMut, MessageInfo };
    use crate::msg::*;
    use crate::contract::*;
    use pix0_contract_common::msg::InstantiateMsg;
    use pix0_contract_common::state::*;
   // use crate::ins::*;
    use crate::query::*;
    use crate::ins::*;


    const DEFAULT_PRICE_DENOM : &str = "uconst";


    fn inst (deps : DepsMut,info: MessageInfo, owner : &str) {

      
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
                Fee {name : "CREATE_BUY_OFFER_FEE".to_string(),
                value : Coin { amount : Uint128::from(2800u64), denom : "uconst".to_string()}},
            ]) ,
            log_last_payment : Some(true)

        };

        let res = instantiate(deps, mock_env(), info.clone(), ins.clone());
       
        println!("Instantiated::{:?}\n", res);
       

    }

    fn loop_create_so(mut deps : DepsMut, info: MessageInfo, max : u64, owner : &str, running_offer_id : bool ) {

        for x in 0..(max+1) {
     
            let tid = format!("Tk_00{}", x);
            let mut oid :Option<String> = None ;

            if running_offer_id {

                oid = Some(format!("Offer_00{}", x));
            }

            let price : Coin = Coin {
                amount : Uint128::from((3500 * (x+1)) as u64 ),
                denom : DEFAULT_PRICE_DENOM.to_string(), 
            };

            let s = SellOffer {
                token_id : tid, 
                owner : Addr::unchecked(owner), 
                collection_info : None,
                offer_id : oid, 
                price : price, 
                status : 0,
                allowed_direct_buy : true, 
                deal_close_type : None,
                date_created : None,
                date_updated : None, 
            };

            let create_so = ExecuteMsg::CreateSellOffer {
                offer : s.clone()
            };
  
             let _res = execute(deps.branch(), mock_env(), info.clone(), 
             create_so.clone());
  
             if _res.is_err() {
  
                 println!("Error.creating so:{}, error:is::{:?}", s.token_id, _res);
             }
             /*
             else {

                println!("{}.Created.so:{:?}", x, _res);
             } */

        }

    }
   
    // cargo test test_create_sell_offers -- --show-output
    #[test]
    fn test_create_sell_offers(){

        println!("Test.create.sell.offers!");

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        inst(deps.as_mut(), info.clone(), owner);
       
        loop_create_so(deps.as_mut(), info.clone(),10, owner, false);

        let res = remove_sell_offer(deps.as_mut(),  info.clone(), String::from("Tk_005"));

        println!("Removed.res:{:?}\n",res);

        let res = get_sell_offers_of(deps.as_ref(), Addr::unchecked(owner), None, None, None);

        for (index, o) in res.ok().unwrap().offers.iter().enumerate() {

            println!("{}: Offer.id::{:?}::tok.id:{:?}",(index+1), o.offer_id, o.token_id);
        }

    
        let oid = String::from("OF4282D6668CBCC8EF");

        let o = internal_get_sell_offer_by_id(deps.as_ref(), oid.clone());

        assert_eq!(oid, o.unwrap().offer_id.unwrap());
        
    }


    fn create_bo(deps : DepsMut, info : MessageInfo, owner : &str, sell_offer_id : String,
    price : Coin ){

        let b = BuyOffer {
            owner : Addr::unchecked(owner), 
            sell_offer_id : sell_offer_id.clone(),
            price : price, 
            accepted : false, 
            date_created : None,
            date_updated : None, 
        };

        let _res = internal_create_buy_offer(deps, mock_env(), info, Addr::unchecked(owner),b, sell_offer_id.clone());

        if _res.is_err() {

            println!("Error.creating BuyOffer for :{}, error:is::{:?}", sell_offer_id, _res);
        }
        else {

            println!("Buy offer.created:::{:?}\n\n", _res);
        }
        /* 
        let create_bo = ExecuteMsg::CreateBuyOffer {
            buy_offer : b.clone(),
            sell_offer_id : sell_offer_id.clone(),
        };
        let _res = execute(deps, mock_env(), info.clone(), 
        create_bo);

        if _res.is_err() {

            println!("Error.creating BuyOffer for :{}, error:is::{:?}", sell_offer_id, _res);
        }*/

    }

    fn loop_create_buy_offers(mut deps : DepsMut, info : MessageInfo, sell_offer_id : String ) {

        let owners = vec!["Bob", "Alice", "Janice"];

        for (i,o) in owners.iter().enumerate() {

            let price = Coin {
                amount : Uint128::from(1200u64 * ((i+1) as u64)),
                denom : DEFAULT_PRICE_DENOM.to_string(),
            };
            create_bo(deps.branch(), info.clone(), o, sell_offer_id.clone(), price);
        }

    }


    // cargo test test_create_buy_offers -- --show-output
    #[test]
    fn test_create_buy_offers(){

        println!("Test.create.buy.offers!");

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        inst(deps.as_mut(), info.clone(), owner);
        
        loop_create_so(deps.as_mut(), info.clone(),3, owner, true);

        let res = get_sell_offers_of(deps.as_ref(), Addr::unchecked(owner), None, None, None);

        for (index, o) in res.ok().unwrap().offers.iter().enumerate() {

            println!("{}: Offer.id::{:?}::tok.id:{:?}",(index+1), o.offer_id, o.token_id);
        }


        let soid = String::from("Offer_002");
        loop_create_buy_offers(deps.as_mut(),info.clone(), soid.clone());

        let soid2 = String::from("Offer_003");
        loop_create_buy_offers(deps.as_mut(),info.clone(), soid2.clone());

        let res = get_buy_offers_of(deps.as_ref(), 
        Addr::unchecked("Bob"), None, None, None);

        if res.is_ok() {
            
            println!("\nBuy offers by Bob::");

            for (i,b) in res.ok().unwrap().offers.iter().enumerate() {

                println!("{}.b::{:?}\n",(i+1), b);
            }
        }
        
        let res = get_buy_offers_by(deps.as_ref(), soid2.clone(), None, None, None);

        if res.is_ok() {

            println!("\nBuy offers by sell offer id:{}::", soid2);

            for (i,b) in res.ok().unwrap().offers.iter().enumerate() {

                println!("{}.b::{:?}\n",(i+1) , b);
                assert_eq!(b.sell_offer_id, soid2 );
            }
        }
        

    }



      // cargo test test_send_to_escrow -- --show-output
      #[test]
      fn test_send_to_escrow(){
  
          let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";
  
          let mut deps = mock_dependencies_with_balance(&coins(12, DEFAULT_PRICE_DENOM));
          let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));
          let balance = get_balance_of_escrow(deps.as_ref(), mock_env(), DEFAULT_PRICE_DENOM);
          println!("Balance before sent:{:?}", balance);

          let amt = Coin { amount : Uint128::from(1280u64), denom : DEFAULT_PRICE_DENOM.to_string()};


          let tx = ExecuteMsg::TestTransferToEscrow { coin : amt};

          let res = execute(deps.as_mut(), mock_env(), info.clone(), tx.clone());
        
          println!("Sent.to.escrow:{:?}", res);

          let balance = get_balance_of_escrow(deps.as_ref(), mock_env(), DEFAULT_PRICE_DENOM);
          println!("Balance after sent:{:?}", balance);

        
      }

}