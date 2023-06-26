use near_sdk::{ext_contract, Gas, AccountId };
use near_sdk::json_types::{U128};

pub const GAS_FOR_FT_TRANSFER:      Gas = Gas(100000000000);
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(2000000000000);

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&self, account_id: AccountId);
    fn storage_deposit(&self, account_id: AccountId);
}

