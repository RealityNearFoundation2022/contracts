use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, Balance};
use near_sdk::{AccountId, PanicOnDefault, Promise};

use near_sdk::PromiseResult;
use serde_json::json;

use crate::utils::{ext_fungible_token, GAS_FOR_FT_TRANSFER, GAS_FOR_RESOLVE_TRANSFER};
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    token: AccountId,
    amount: u128,
    amount_claimed: u128,
    start: u64,
    duration: u64,
    is_active: bool,
}

/*
    Implementation of vesting contract

    References:
    https://github.com/JoinColony/colonyToken81jajFoRPjRar5i8yQ6RyiqvziTqnxPJLCvg1C7D3tY181jajFoRPjRar5i8yQ6RyiqvziTqnxPJLCvg1C7D3tY1/blob/master/contracts/Vesting.sol
    https://github.com/cpu-coin/CPUcoin/blob/master/contracts/ERC20Vestable.sol
    https://github.com/dreamteam-gg/smart-contracts/blob/master/contracts/vesting/DreamTokensVesting.sol
    https://modex.tech/developers/OpenZeppelinTeam/OpenZeppelin/src/master/contracts/drafts/TokenVesting.sol
*/
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: AccountId,
        token: AccountId,
        amount: U128,
        start: U64,
        duration: U64,
    ) -> Self {
        assert!(duration.0 > 0, "ERR_DURATION_IS_LESS_THAN_ZERO");
        assert!(
            (start
                .0
                .checked_add(duration.into())
                .expect("ERR_INTEGER_OVERFLOW"))
                > env::block_timestamp().into(),
            "ERR_START_AND_DURATION_IS_IN_THE_PAST"
        );
        let this = Self {
            owner: owner.into(),
            token: token.into(),
            amount: amount.into(),
            amount_claimed: 0,
            start: start.0,
            duration: duration.0,
            is_active: true,
        };
        this
    }

    pub fn owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn amount(&self) -> U128 {
        self.amount.into()
    }

    pub fn token(&self) -> AccountId {
        self.token.clone()
    }

    pub fn amount_claimed(&self) -> U128 {
        self.amount_claimed.into()
    }

    pub fn start(&self) -> U64 {
        self.start.into()
    }

    pub fn duration(&self) -> U64 {
        self.duration.into()
    }

    pub fn deactivate(&mut self) {
        // Comprueba que el invocador de la función es el propietario del contrato
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the owner can deactivate the contract"
        );
        // Cambia is_active a false
        self.is_active = false;
    }

    #[payable]
    #[private]
    pub fn transfer(&mut self, owner_id: AccountId, token_amount: u128) -> Promise {
        let _attached_deposit = env::attached_deposit();
        // assert_eq!(attached_deposit, 1, "Requires attached deposit of exactly 1 yoctoNEAR");

        let promise = ext_fungible_token::ext(self.token.clone())
            .with_static_gas(GAS_FOR_FT_TRANSFER)
            .with_attached_deposit(1 /* default deposit of 0 */)
            .ft_transfer(owner_id.clone(), token_amount.into(), None);
        promise
    }

    #[payable]
    pub fn storage_deposit(&mut self) -> Promise {
        // Get who is calling the method
        let owner_id: AccountId = env::predecessor_account_id();

        let attached = env::attached_deposit();

        ext_fungible_token::ext(self.token.clone())
            .with_attached_deposit(attached)
            .storage_deposit(owner_id.clone())
    }

    #[payable]
    pub fn buy(&mut self, package: u8, quantity: u8) -> Promise {
        assert!(
            self.is_active,
            "La compra no está permitida porque el contrato no está activo"
        );
        let predecessor = env::predecessor_account_id();
        // let initial_balance = env::account_balance();
        let owner_id: AccountId = env::predecessor_account_id();
        // Si la cantidad es 0, se establece en 1
        let quantity = if quantity == 0 { 1 } else { quantity };
        let (token_price, token_amount): (u128, u128) = match package {
            1 => (100 * 10u128.pow(24), 1000 * 10u128.pow(8)),
            2 => (500 * 10u128.pow(24), 7500 * 10u128.pow(8)),
            3 => (1500 * 10u128.pow(24), 30000 * 10u128.pow(8)),
            _ => panic!("Paquete no válido"),
        };
        // Multiplicar el precio y la cantidad de tokens por la cantidad de paquetes
        let total_price = token_price
            .checked_mul(quantity as u128)
            .expect("ERR_INTEGER_OVERFLOW");
        let total_amount = token_amount
            .checked_mul(quantity as u128)
            .expect("ERR_INTEGER_OVERFLOW");

        assert!(
            self.amount >= total_amount,
            "La cantidad solicitada excede la cantidad disponible"
        );

        let attached = env::attached_deposit();
        assert_eq!(
            attached, total_price,
            "El depósito adjunto no coincide con el precio del paquete"
        );

        self.amount_claimed = self
            .amount_claimed
            .checked_add(total_amount)
            .expect("ERR_INTEGER_OVERFLOW");

        self.amount = self
            .amount
            .checked_sub(total_amount)
            .expect("ERR_INTEGER_UNDERFLOW");

        // Serializa los datos necesarios en formato JSON
        let serialized_data = serde_json::to_string(&ResolveTransferArgs {
            predecessor: predecessor.clone(),
            initial_balance: U128::from(attached),
        })
        .unwrap();

        let transfer_promise = self.transfer(owner_id, total_amount);

        transfer_promise
            .then(
                Promise::new(env::current_account_id())
            )
            .function_call(
                "resolve_transfer".to_string(),
                json!({"args": serialized_data.into_bytes()}).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_RESOLVE_TRANSFER,
            )
    }

    #[private]
    pub fn resolve_transfer(&mut self, args: Vec<u8>) {
        // Deserializa los datos necesarios desde una cadena JSON
        let args_str = String::from_utf8(args).unwrap();
        let ResolveTransferArgs {
            predecessor,
            initial_balance,
        }: ResolveTransferArgs = serde_json::from_str(&args_str).unwrap();

        if let PromiseResult::Failed = env::promise_result(0) {
            Promise::new(predecessor).transfer(initial_balance.0);
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResolveTransferArgs {
    predecessor: AccountId,
    initial_balance: U128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Setup the initial state of the contract
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }
    // Helper function to simulate contract deployment
    fn setup_contract() -> Contract {
        let context = get_context(vec![], false);
        testing_env!(context);
        Contract::new(
            "alice.testnet".to_string(),
            "token.testnet".to_string(),
            1000000000000000000000000.into(),
            1629055854000000000.into(),
            31556952000000000.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    // ... (las importaciones y funciones de ayuda van aquí)

    #[test]
    fn new() {
        let contract = setup_contract();
        assert_eq!(contract.owner, "alice.testnet");
        assert_eq!(contract.token, "token.testnet");
        assert_eq!(contract.amount, 1000000000000000000000000);
        assert_eq!(contract.start, 1629055854000000000);
        assert_eq!(contract.duration, 31556952000000000);
        assert!(contract.is_active);
    }

    #[test]
    fn owner() {
        let contract = setup_contract();
        assert_eq!(contract.owner(), "alice.testnet");
    }

    #[test]
    fn amount() {
        let contract = setup_contract();
        assert_eq!(contract.amount().0, 1000000000000000000000000);
    }

    #[test]
    fn token() {
        let contract = setup_contract();
        assert_eq!(contract.token(), "token.testnet");
    }

    #[test]
    fn amount_claimed() {
        let contract = setup_contract();
        assert_eq!(contract.amount_claimed().0, 0);
    }
    // ... (más tests aquí)
}
