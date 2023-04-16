## Instantiate the contract
```archway instantiate -c 529 --args '{"fees":[{"name":"CREATE_SELL_OFFER_FEE", "value":{"amount":"3200","denom":"uconst"}},{"name":"CREATE_BUY_OFFER_FEE", "value":{"amount":"2600","denom":"uconst"}}] }' ```


## Query the contract info
```archway query contract-state smart --args '{"get_contract_info": {}}'  ```

