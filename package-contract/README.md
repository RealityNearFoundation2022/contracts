# Packages Token Smart Contract

This is the codebase for the Package Token smart contract.

The smart contract was built using NEAR Protocol and it is intended to allow the purchase of different packages of tokens, with its own features and functionalities.

## Building this contract

```bash
./build.sh or
yarn build
```

## Features

The main features of this contract include:

    - Initialization of the contract with the owner, token, amount, start, and duration parameters.
    - Ability to get information about the owner, amount, token, claimed amount, start, and duration.
    - Ability to make a payment and transfer a specific amount of tokens to an owner.
    - Capability of depositing to storage.
    - Capability to purchase packages with an added parameter for the quantity of packages to buy.

## Usage

To use this smart contract, you need to initialize it with new, and provide the parameters for owner, token, amount, start, and duration.

### Initialization

```bash
#[init]
pub fn new(
    owner: AccountId,
    token: AccountId,
    amount: U128,
    start: U64,
    duration: U64,
) -> Self;
```

### Get Methods

The following getter methods are available:

    - `owner`: Returns the owner of the contract.
    - `amount`: Returns the total amount of tokens available.
    - `token`: Returns the token ID.
    - `amount_claimed`: Returns the total amount of tokens claimed.
    - `start`: Returns the start timestamp.
    - `duration`: Returns the duration of the contract.

### Purchase

To purchase a package of tokens:

```bash
#[payable]
pub fn buy(&mut self, package: u8, quantity: u8) -> Promise;
```

The `buy` function accepts the package identifier and the quantity of the package as parameters. This function also checks if the attached deposit matches the package price and transfers the tokens to the owner account.


### Transfer

To transfer a specific amount of tokens to an owner:

```bash
#[payable]
#[private]
pub fn transfer(&mut self, owner_id: AccountId, token_amount: u128) -> Promise;
```

## Example

Here is an example of how you can initialize and interact with the contract:

```bash
TOKEN_SALE="token.guxal.testnet"
CONTRACT="presale.guxal.testnet"
AMOUNT="1250000000000000000000000"

# Initialize the contract
near call $CONTRACT --accountId $CONTRACT new '{"owner":"'$CONTRACT'", "token":"'$TOKEN_SALE'","amount":"'$AMOUNT'","start":"1629055854000000000", "duration":"31556952000000000"}'

# Deposit tokens to the contract
near call $TOKEN_SALE storage_deposit '{"account_id": "'$CONTRACT'"}' --accountId $ADMIN --amount 0.01
near call $TOKEN_SALE ft_transfer '{"receiver_id": "'$CONTRACT'", "amount": "'$AMOUNT'", "memo": ""}' --accountId $TOKEN_SALE --depositYocto 1

# Purchase a package
near call $CONTRACT buy '{"package": 1, "quantity": 1}' --accountId lolabunny.testnet --amount 100
```

Please note that after calling new(), you need to transfer the tokens to the contract using the ft_transfer function.

## Build and Deploy

You can use the NEAR CLI to build the contract. To deploy the contract, you need access to the NEAR TestNet or MainNet, and an account on NEAR.

## Disclaimer

This contract is a demonstration and not intended for production use. If you plan to use this code in a production environment, ensure that it has been thoroughly tested and that the contract logic meets your requirements.