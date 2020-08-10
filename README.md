# sub_swap

* Uniswap like blockchain

## TODO 

### substrate Setup 

[x] Add Asset pallet

### pallet functions

[ ] allow pairs to be created (Im thinking a mapping of token to balance)

[ ] getPairs

[ ] allPairs

[ ] getReserves - balance of token in contract

[ ] mint - creates pool tokens (upon adding liq)

[ ] burn - burns pool token (removing liq)

[ ] swap - trades token 

[ ] skim - meh maybe later

[ ] sync - meh maybe later

### pallet events 
[ ] pair created


Tokens will get traded in comparision to base token very simply by p = t1 / t2 

So in a scenerio where the contract balance is t1 = 10 and t2 = 20  p of t1 = p2 / p1 = 2 


Will be working off this implementaiton https://github.com/PhABC/uniswap-solidity/blob/master/contracts/uniswap/UniswapExchange.sol

Important, seems like prices are calculated before transfer, would this be bad for impermenent loss and slippage, check if that is right with vyper contracts

front end data types 

```
{
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "AssetId": "u128"
}
```