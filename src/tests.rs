#[cfg(test)]
mod tests {
  
    use crate::{state::*, ContractError};
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr,  Coin, Uint128, DepsMut, MessageInfo, Response };
    use crate::msg::*;
    use crate::contract::*;
    use pix0_contract_common::msg::InstantiateMsg;
    use pix0_contract_common::state::*;
   // use crate::ins::*;
    use crate::query::*;
    use crate::ins::*;
    use pix0_market_handlers::state::{SimpleCollectionInfo, Royalty};

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


    fn create_so (mut deps : DepsMut, info: MessageInfo, owner : &str, token_id : String,
    offer_id : Option<String>, price : Coin, contract_addr : String, collection_info : Option<SimpleCollectionInfo>  )
    -> Result<Response, ContractError> {

      
        let s = SellOffer {
            token_id : token_id, 
            owner : Addr::unchecked(owner), 
            collection_info : collection_info,
            contract_addr : contract_addr.clone(),
            offer_id : offer_id, 
            price : price, 
            status : 0 ,
            allowed_direct_buy : true, 
            deal_close_type : None,
            date_created : None,
            date_updated : None, 
        };

        let create_so = ExecuteMsg::CreateSellOffer {
            offer : s.clone()
        };

        execute(deps.branch(), mock_env(), info.clone(), 
         create_so.clone())
    }

    fn loop_create_so(mut deps : DepsMut, info: MessageInfo, max : u64, owner : &str, 
    contract_addr : String, running_offer_id : bool) {

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

            
             let _res = create_so(deps.branch(), 
                info.clone(), owner, tid.clone(), 
                oid, price, contract_addr.clone(), Some(SimpleCollectionInfo {
                    collection_name : format!("XYZ-{} Collection", x),
                    collection_symbol :  format!("XYZ{}", x),
                    owner :Addr::unchecked("Alice"),
                    category : Some(format!("Category_{}",x )), 
                    royalties : None, 
                }));

             if _res.is_err() {
  
                 println!("Error.creating so:{}, error:is::{:?}", tid , _res);
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

        let contract_addr = "cosmoms3contract".to_string(); //mock_env().contract.address;

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        inst(deps.as_mut(), info.clone(), owner);
       
        loop_create_so(deps.as_mut(), info.clone(),10, owner, contract_addr.clone().to_string(), false );

        let res = cancel_sell_offer(deps.as_mut(), mock_env(),  info.clone(), 
        String::from("Tk_005"), contract_addr.to_string());

        println!("Removed.res:{:?}\n",res);

        let res = get_sell_offers_of(deps.as_ref(), 
        Addr::unchecked(owner), None, None, None);

        for (index, o) in res.ok().unwrap().offers.iter().enumerate() {

            println!("{}: Offer.id::{:?}::tok.id:{:?}",(index+1), o.offer_id, o.token_id);
        }

    
        let oid = String::from("OF4B8EF5D1B36F1C84");

        let o = internal_get_sell_offer_by_id(deps.as_ref(), oid.clone());

        assert_eq!(oid, o.unwrap().offer_id.unwrap());


        // get all collection indexes 

        let cat = Some("Category_3".to_string());
        let res = get_collection_indexes(deps.as_ref(), 
        cat.clone(), None, None);

        println!("\nCollections by category :{:?}", cat);

        if res.is_ok() {

            let res = res.unwrap();
            for (i , c) in 
            res.collection_indexes.iter().enumerate() {

                println!("{} : {:?}", i, c);

                let res2 = get_collection_index(deps.as_ref(), 
                c.id.clone());

                if res2.is_ok() {

                    let res2 = res2.unwrap();
                    if res2.collection_index.is_some() {

                        let id = res2.collection_index.unwrap().id;
                        assert_eq!(c.id, id);
                        println!("Id::{}", id);
                    }

                }
            }
        }

        

        
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

        let _res = internal_create_buy_offer(deps, mock_env(), info, Addr::unchecked(owner),b.clone(), sell_offer_id.clone());

        if _res.is_err() {

            println!("Error.creating BuyOffer for :{}, error:is::{:?}", sell_offer_id, _res);
        }
        else {

            println!("Buy offer.created by:::\n{:?}:Price:{:?}\n\n", b.owner, b.price);
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
        
        loop_create_so(deps.as_mut(), info.clone(),3, owner, mock_env().contract.address.to_string(), 
        true);

        let res = get_sell_offers_of(deps.as_ref(), 
        Addr::unchecked(owner), 
        None, None, None);

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

        let tid_to_cancel = String::from("Tk_002");

        let res = cancel_sell_offer(deps.as_mut(),  
        mock_env(), info.clone(), tid_to_cancel.clone(),
        mock_env().contract.address.to_string());
        println!("Cancel.sell.offer:{}.res:\n{:?}", tid_to_cancel, res);
        
        let res = get_buy_offers_by(deps.as_ref(), soid.clone(), None, None, None);

        if res.is_ok() {

            println!("\nBuy offers by sell offer id:{}::", soid);

            for (i,b) in res.ok().unwrap().offers.iter().enumerate() {

                println!("{}.b::{:?}\n",(i+1) , b);
                assert_eq!(b.sell_offer_id, soid );
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


       // cargo test test_accept_buy_offer -- --show-output
    #[test]
    fn test_accept_buy_offer(){

        println!("Test.create.sell.offers!");

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let contract_addr = "cosmos3contract".to_string(); //mock_env().contract.address;

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        inst(deps.as_mut(), info.clone(), owner);

        let tid = String::from("Tok_009x");
        let oid = Some(String::from("Oid_009x"));

        let price : Coin = Coin {
            amount : Uint128::from(3500u64 ),
            denom : DEFAULT_PRICE_DENOM.to_string(), 
        };

        let _res = create_so(deps.as_mut(), 
            info.clone(), owner, tid, 
            oid.clone(), price.clone(), contract_addr, Some(SimpleCollectionInfo {
                collection_name : format!("XYZ-{} Collection", 1),
                collection_symbol :  format!("XYZ{}", 1),
                owner :Addr::unchecked("Alice"),
                category : Some(format!("Category_{}",1 )), 
                royalties : Some(vec![Royalty {
                    name : None,
                    wallet : Addr::unchecked("Sarah"),
                    percentage : 500, // 5%
                },Royalty {
                    name : None,
                    wallet : Addr::unchecked("John"),
                    percentage : 255, // 2.55%
                }]), 
        }));

        let _res = loop_create_buy_offers(deps.as_mut(),info.clone(), oid.clone().unwrap());

        let _res = accept_buy_offer(deps.as_mut(), mock_env(), 
        info, Addr::unchecked("Alice"), oid.unwrap());

        println!("Buy offer accepted.result : {:?}", _res);

    }

}