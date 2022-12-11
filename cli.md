## I. Environment Setup
### 1. Setup testnet account that you own access key:
```bash
export TGAS=000000000000 && \
export USER_ID=andreapn1709.testnet && \
export USER_ID2=samoykute.testnet && \
export USER_ID3=lovelyfifi.testnet
```
### 2. Setup token address:
```bash
export WNEAR=wrap.testnet && \
export REF_ID=ref.fakes.testnet && \
export DAI_ID=dai.fakes.testnet && \
export ETH_ID=eth.fakes.testnet && \
export SKYWARD_ID=skyward.fakes.testnet
```

### 3. Setup contract address:
```bash
export REF=ref-finance-101.testnet && \
export REF_FARM=boostfarm.ref-finance.testnet
```
### 4. Build and deploy contract:
```bash
yarn deploy
# Get testnet address after build (example: dev-1670572742038-99993572685800):
export CONTRACT_ID=dev-1670572742038-99993572685800
```

## II. Init Contract:
### 1. Storage Deposit:
```bash
near call $SKYWARD_ID storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $WNEAR storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $REF_ID storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $ETH_ID storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $DAI_ID storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $REF storage_deposit --amount 0.1 --accountId $CONTRACT_ID && \
near call $REF_FARM storage_deposit --amount 0.1 --accountId $CONTRACT_ID 
```

### 2. Register Token:
```bash
near call $REF register_tokens '{"token_ids": ["'$REF_ID'","'$WNEAR'","'$DAI_ID'","'$ETH_ID'","'$SKYWARD_ID'"]}' --accountId $CONTRACT_ID --depositYocto 1
```

### 3. View pool by pool id:
```bash
near view $REF get_pool '{"pool_id":50}'
```

### 4. Deposit token:
```bash
near call $ETH_ID ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount":"1142356271134177","msg":""}' --accountId $USER_ID --depositYocto 1 --gas 80$TGAS

near call $REF_ID ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount":"2099656072404060000000000","msg":""}' --accountId $USER_ID2 --depositYocto 1 --gas 35$TGAS

near call $SKYWARD_ID ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount":"2000000000000003693","msg":""}' --accountId $USER_ID2 --depositYocto 1 --gas 35$TGAS

near call $WNEAR ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount":"4984347565080580000000000","msg":""}' --accountId $USER_ID2 --depositYocto 1 --gas 35$TGAS

near call $DAI_ID ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount":"82877254348332300000","msg":""}' --accountId $USER_ID --depositYocto 1 --gas 80$TGAS
```

### 5. Check deposited balance:
``` bash
near view $CONTRACT_ID get_balance '{"account_id":"'$USER_ID'"}'
```

### 6. Call add liquidity:
```bash
near call $CONTRACT_ID call_add_liquidity '{"pool_id":17,"amounts":["2099656072404060000000000","4984347565080580000000000"],"token1":"'$REF_ID'", "token2": "'$WNEAR'"}' --accountId $USER_ID2 --gas 300$TGAS
```

### 7. Call remove liquidity:
```bash
near call $CONTRACT_ID call_remove_liquidity '{"pool_id":17,"min_amounts":["4822648547890330000000000","11472992805501600000000000"],"shares":"8312577166003880000000000000","token1":"'$REF_ID'", "token2": "'$WNEAR'"}' --accountId $USER_ID2 --gas 300$TGAS && \
near call $CONTRACT_ID withdraw_max '{"token_id":"'$REF_ID'"}' --accountId $USER_ID2 --gas 100$TGAS && \
near call $CONTRACT_ID withdraw_max '{"token_id":"'$WNEAR'"}' --accountId $USER_ID2 --gas 100$TGAS
```

## III. Add & remove farm:
```bash
near call $CONTRACT_ID call_mft_transfer_call '{"amount":"800000000000000000000000","pool_id":":50"}' --accountId $USER_ID2 --gas 300$TGAS

near call $CONTRACT_ID call_unlock_and_withdraw_seed '{"seed_id":"ref-finance-101.testnet@17","amount":"5000000000000000000000000000"}' --accountId $USER_ID2 --gas 300$TGAS
```

## IV. Claim Contract FARM reward:
``` bash
near call $CONTRACT_ID call_claim_reward_by_seed '{"seed_id":"ref-finance-101.testnet@50"}' --accountId $USER_ID2 --gas 300$TGAS

# Get reward in withdraw box
near view $REF_FARM list_farmer_rewards '{"farmer_id":"'$CONTRACT_ID'"}'

near call $CONTRACT_ID call_withdraw_reward '{"token_id":"ref.fakes.testnet"}' --accountId $USER_ID --gas 300$TGAS
```


### Some manual CLI on REF an tokens:
```bash
near call $REF remove_liquidity '{"min_amounts": ["594352005602050000000000","1373425444667410000000000"],"pool_id": 17, "shares": "1009680522490590000000000000"}' --accountId $CONTRACT_ID --depositYocto 1 --gas 80$TGAS

near call $REF withdraw '{"token_id": "'$REF_ID'", "amount": "0", "unregister": false}' --accountId $CONTRACT_ID --depositYocto 1 --gas 60$TGAS

near call $REF withdraw '{"token_id": "'$WNEAR'", "amount": "1733029503802090316150369"}' --accountId $CONTRACT_ID --depositYocto 1 --gas 80$TGAS

near call $REF_ID ft_transfer '{"receiver_id":"'$USER_ID'","amount":"753312227935478675274293"}' --accountId $CONTRACT_ID --depositYocto 1 --gas 80$TGAS

near call $WNEAR ft_transfer '{"receiver_id":"'$USER_ID'","amount":"1733029503802090316150369"}' --accountId $CONTRACT_ID --depositYocto 1 --gas 80$TGAS

near view $REF get_pool_shares '{"pool_id": 50, "account_id": "'$CONTRACT_ID'"}'

near view $CONTRACT_ID get_pools '{"account_id":"'$USER_ID2'"}'

near view $CONTRACT_ID get_farms '{"account_id":"'$USER_ID2'"}'

near view $REF_FARM get_farmer_seed '{"farmer_id":"'$CONTRACT_ID'","seed_id":"ref-finance-101.testnet@50"}'

near call $CONTRACT_ID modify_pool '{"account_id":"'$USER_ID'","pool_id":"17","shares":"800000000000000000000000000"}' --accountId $USER_ID 

near call $CONTRACT_ID modify_farm '{"account_id":"'$USER_ID2'","farm_id":"50","amount":"981592273212832991262629"}' --accountId $USER_ID

near view $REF_FARM list_seeds_info ''

# Get unreward in seed
near view $REF_FARM get_unclaimed_rewards '{"farmer_id": "'$CONTRACT_ID'", "seed_id": "ref-finance-101.testnet@50"}'

near call $REF_FARM claim_reward_by_seed '{"seed_id":"ref-finance-101.testnet@50"}' --accountId $USER_ID2 --gas 35$TGAS

near call $REF_FARM withdraw_reward '{"token_id":"ref.fakes.testnet"}' --accountId $USER_ID2 --gas 45$TGAS

near call $REF_FARM unlock_and_withdraw_seed '{"seed_id":"ref-finance-101.testnet@50", "unlock_amount": "0", "withdraw_amount": "200000000000000000000000"}' --accountId $USER_ID2 --depositYocto 1 --gas 45$TGAS
```

