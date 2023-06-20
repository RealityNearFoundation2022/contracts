use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, Balance};
use near_sdk::{AccountId, PanicOnDefault, Promise};

use crate::utils::{ext_fungible_token, GAS_FOR_FT_TRANSFER};
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
        let attached = env::attached_deposit();

        assert_eq!(
            attached, total_price,
            "El depósito adjunto no coincide con el precio del paquete"
        );

        self.amount_claimed = self
            .amount_claimed
            .checked_add(total_amount)
            .expect("ERR_INTEGER_OVERFLOW");
        self.transfer(owner_id, total_amount)
    }

}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    const _ONE_PARAS_TOKEN: U128 = U128(1 * 10u128.pow(18));
    const _TEN_PARAS_TOKEN: U128 = U128(10 * 10u128.pow(18));
    const _TEN_MILLION_PARAS_TOKEN: U128 = U128(10_000_000 * 10u128.pow(18));
    const FIVE_HUNDRED_THOUSAND_PARAS_TOKEN: U128 = U128(500_000 * 10u128.pow(18));
    const TOTAL_AMOUNT: U128 = FIVE_HUNDRED_THOUSAND_PARAS_TOKEN;

    // IN NANO SECONDS
    const ONE_MONTH: u64 = 2629746000000000; // 30.436875*24*60*60*10**9
    const TWO_YEARS: u64 = ONE_MONTH * 12 * 2;
    const JUNE_1_2021: u64 = 1622505600000000000; // Tuesday, June 1, 2021 12:00:00 AM GMT
    const ONE_DAY: u64 = 86400000000000;
    const SIX_MONTHS: u64 = ONE_MONTH * 6;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new(
            accounts(1).into(),
            accounts(3).into(),
            accounts(2).into(),
            TOTAL_AMOUNT,
            U64::from(JUNE_1_2021),
            U64::from(TWO_YEARS),
            U64::from(SIX_MONTHS),
            true,
        );
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1).into(),
            accounts(3).into(),
            accounts(2).into(),
            TOTAL_AMOUNT,
            U64::from(JUNE_1_2021),
            U64::from(TWO_YEARS),
            U64::from(SIX_MONTHS),
            false,
        );
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.owner(), accounts(1).to_string());
        assert_eq!(contract.recipient(), accounts(3).to_string());
        assert_eq!(contract.token(), accounts(2).to_string());
        assert_eq!(contract.amount(), TOTAL_AMOUNT);
        assert_eq!(contract.amount_claimed(), U128(0));
        assert_eq!(contract.start(), U64::from(JUNE_1_2021));
        assert_eq!(contract.cliff(), U64::from(JUNE_1_2021 + SIX_MONTHS));
        assert_eq!(contract.duration(), U64::from(TWO_YEARS));
        assert_eq!(contract.revocable(), false);
        assert_eq!(contract.is_active, true);
    }

    #[test]
    fn test_calculate_amount_vested() {
        let (mut context, contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(1618109122863866400)
            .build());
        let amount_vested = contract.calculate_amount_vested();
        assert_eq!(amount_vested, U128::from(0));

        // after start before cliff ONE DAY
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.start + ONE_DAY)
            .build());
        let amount_vested = contract.calculate_amount_vested();
        assert_eq!(amount_vested, U128::from(0));

        // after start before cliff ONE MONTH
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.start + ONE_MONTH)
            .build());
        let amount_vested = contract.calculate_amount_vested();
        assert_eq!(amount_vested, U128::from(0));

        // after cliff after ONE_DAY*29
        // month -> 0
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_DAY * 29)
            .build());
        let amount_vested: u128 = contract.calculate_amount_vested().into();
        assert_eq!(amount_vested, TOTAL_AMOUNT.0 * 6 / 24);

        // after cliff after ONE MONTH
        // (FIVE_HUNDRED_THOUSAND_PARAS / (contract.duration / ONE_MONTH)) == 20833333333333333333333333333 == 20833.333333333332 PARAS/month
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .build());
        let amount_vested: u128 = contract.calculate_amount_vested().into();
        assert_eq!(amount_vested, TOTAL_AMOUNT.0 * 7 / 24);

        // after cliff after ONE MONTH + 29 Days
        // (FIVE_HUNDRED_THOUSAND_PARAS / (contract.duration / ONE_MONTH)) == 20833333333333333333333333333 == 20833.333333333332 PARAS/month
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH + ONE_DAY * 29)
            .build());
        let amount_vested: u128 = contract.calculate_amount_vested().into();
        assert_eq!(amount_vested, TOTAL_AMOUNT.0 * 7 / 24);

        // after cliff after duration (vesting over)
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + contract.duration + 1)
            .build());
        let amount_vested = contract.calculate_amount_vested();
        assert_eq!(amount_vested, TOTAL_AMOUNT);
    }

    #[test]
    fn test_claim_vested() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff - 1)
            .build());
        let releasable_amount = contract.internal_releasable_amount();
        assert_eq!(releasable_amount, 0);

        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .build());
        let releasable_amount: u128 = contract.internal_releasable_amount().into();
        assert_eq!(releasable_amount, TOTAL_AMOUNT.0 * 7 / 24);

        // claim
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .attached_deposit(1)
            .build());
        contract.claim_vested();
        assert_eq!(contract.amount_claimed, TOTAL_AMOUNT.0 * 7 / 24);

        // the next month
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH * 2)
            .build());
        let releasable_amount = contract.internal_releasable_amount();
        assert_eq!(releasable_amount, TOTAL_AMOUNT.0 * 1 / 24);

        // claim
        contract.claim_vested();
        assert_eq!(contract.amount_claimed, TOTAL_AMOUNT.0 * 8 / 24);

        // after vesting period over
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + contract.duration + 1)
            .build());

        let amount_vested = contract.calculate_amount_vested();
        assert_eq!(amount_vested, TOTAL_AMOUNT);

        let releasable_amount = contract.internal_releasable_amount();
        assert_eq!(
            releasable_amount,
            u128::from(TOTAL_AMOUNT) - TOTAL_AMOUNT.0 * 8 / 24
        );

        contract.claim_vested();
        assert_eq!(contract.amount_claimed, u128::from(TOTAL_AMOUNT));

        // after claim everything
        let releasable_amount = contract.internal_releasable_amount();
        assert_eq!(releasable_amount, 0);
    }

    #[test]
    fn test_revoke() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .build());
        let releasable_amount = contract.internal_releasable_amount();
        assert_eq!(releasable_amount, TOTAL_AMOUNT.0 * 7 / 24);

        // claim
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .attached_deposit(1)
            .build());
        contract.claim_vested();
        assert_eq!(contract.amount_claimed, TOTAL_AMOUNT.0 * 7 / 24);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .block_timestamp(contract.cliff + ONE_MONTH)
            .attached_deposit(1)
            .build());

        let current_amount_claimed = contract.amount_claimed();
        let releasable_amount = contract.internal_releasable_amount();
        // revoke
        let amount_not_vested = contract.revoke();
        assert_eq!(
            amount_not_vested,
            U128::from(
                u128::from(TOTAL_AMOUNT)
                    - u128::from(current_amount_claimed)
                    - u128::from(releasable_amount)
            )
        );

        assert_eq!(contract.is_active, false);
        // assert_eq!(contract.recipient(), accounts(1).to_string());
        assert_eq!(contract.amount, 0);
        assert_eq!(contract.start, 0);
        assert_eq!(contract.duration, 0);
        assert_eq!(contract.cliff, 0);
    }
    // NEGATIVE
    #[test]
    #[should_panic(expected = "ERR_NO_VESTED_AMOUNT_ARE_DUE")]
    fn test_invalid_claim_vested() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(3))
            .block_timestamp(contract.cliff - 1)
            .attached_deposit(1)
            .build());
        contract.claim_vested();
    }
    #[test]
    #[should_panic(expected = "ERR_CALLER_NOT_RECIPIENT")]
    fn test_invalid_claim_vested_caller_not_recipient() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(4))
            .block_timestamp(contract.cliff + contract.duration)
            .attached_deposit(1)
            .build());
        contract.claim_vested();
    }

    #[test]
    fn test_change_recipient() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context.predecessor_account_id(accounts(1)).build());

        contract.change_recipient("changed.near".to_string());

        assert_eq!(contract.recipient(), "changed.near");
    }
}
