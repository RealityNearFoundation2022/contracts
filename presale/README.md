PARAS Vesting Contract
==============

## Building this contract
```bash
yarn build
```

## Using this contract

### Quickest deploy
```bash
yarn dev
```

## Testing
To test run:
```bash
yarn test
```

# Contract functions

## View methods

### Get Recipient

```
recipient()
```

### Get Amount

```
amount()
```

### Get Amount claimed

```
amount_claimed()
```

### Get cliff (start + cliff) in nano seconds

```
cliff()
```

### Get start (in nano seconds)

```
start()
```

### Get duration (in nano seconds)

```
duration()
```

### Get revocable

```
revocable()
```

###  Get releasable_amount (amount releasable to recipient at current blockchain timestamp)
```
releasable_amount()
```

### Get amount_vested (total amount vested at current blockchain timestamp)

```
calculate_amount_vested()
```

## Call methods

### New 
```
near call dev-1631684538328-15645042144806 --accountId dev-1631684538328-15645042144806 new '{"owner":"dev-1631684538328-15645042144806", "recipient":"rng.testnet","token":"dev-1631277489384-75412609538902","amount":"1250000000000000000000000","start":"1629055854000000000", "duration":"31556952000000000", "cliff_duration":"0", "revocable":false}'
```

NOTE: after calling new(), do ft_transfer of PARAS to vesting_contract\
NOTE: the recipient must register on PARAS FT contract to obtain tokens\

### Claim vested

```
claim_vested()
```

### Revoke - Owner Only (revocable == true)
```
revoke({"recipient":"alice.testnet"})
```


near call $CONTRACT --accountId $CONTRACT new '{"owner":"guxal.testnet", "recipient":"bob.guxal.testnet","token":"$CONTRACT", "token_price": 10, "amount":"100000","start":"1674506706000000000", "duration":"31556952000000000", "cliff_duration":"0", "revocable":false}'

near call token.guxal.testnet ft_transfer '{"receiver_id": "guxal.testnet", "amount": "100", "memo": "start :)"}' --accountId token.guxal.testnet --depositYocto 1

429618989789786

near call token.guxal.testnet storage_deposit '{"account_id": "dev-1675634479426-76608507847363"}' --accountId guxal.testnet --amount 0.01


near call dev-1675634479426-76608507847363 buy '{}' --accountId lolabunny.testnet --amount 1

near call token.guxal.testnet storage_deposit '{"account_id": "guxal.testnet"}' --accountId guxal.testnet --amount 0.01

near view token.guxal.testnet ft_balance_of '{"account_id": "'dev-1675634479426-76608507847363'"}'

dev-1675634479426-76608507847363


near call dev-1675634479426-76608507847363 change_token_price '{"price": "1000000000000000"}' --accountId guxal.testnet


near view token.guxal.testnet ft_balance_of '{"account_id": "'token.guxal.testnet'"}'

near view dev-1675634479426-76608507847363 amount_claimed


250_000_000_000_000_000_000_000

near call dev-1675634479426-76608507847363 storage_deposit '' --accountId lolabunny.testnet --amount 0.00125

near call token.guxal.testnet storage_deposit '' --accountId lolabunny.testnet --amount 0.00125

near call token.guxal.testnet storage_unregister '{ "force": true }' --accountId lolabunny.testnet --depositYocto 1

near view token.guxal.testnet storage_balance_of '{"account_id": "guxal.testnet"}'

------------

near call token.guxal.testnet storage_deposit '{"account_id": "'$CONTRACT'"}' --accountId guxal.testnet --amount 0.01

near call $ID_TOKEN ft_transfer '{"receiver_id": "'$CONTRACT'", "amount": "1000000000"}' --accountId guxal.testnet --amount 0.000000000000000000000001

near view token.guxal.testnet ft_balance_of '{"account_id": "'dev-1675634479426-76608507847363'"}'


near call dev-1675634479426-76608507847363 change_token_price '{"price": "1000000000000000"}' --accountId guxal.testnet


near view token.guxal.testnet ft_balance_of '{"account_id": "'token.guxal.testnet'"}'

near view dev-1675634479426-76608507847363 amount_claimed


250_000_000_000_000_000_000_000

near call dev-1675634479426-76608507847363 storage_deposit '' --accountId lolabunny.testnet --amount 0.00125

near call token.guxal.testnet storage_deposit '' --accountId lolabunny.testnet --amount 0.00125

near call token.guxal.testnet storage_unregister '{ "force": true }' --accountId lolabunny.testnet --depositYocto 1

near view token.guxal.testnet storage_balance_of '{"account_id": "guxal.testnet"}'