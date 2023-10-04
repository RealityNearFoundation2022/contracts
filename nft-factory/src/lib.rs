// use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::env::STORAGE_PRICE_PER_BYTE;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
};
// use near_sdk::PromiseResult;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
// NonFungibleTokenMetadataProvider NFT_METADATA_SPEC Gas
near_sdk::setup_alloc!();

// const NFT_WASM_CODE: &[u8] = include_bytes!("./nft-contract/nft_simple.wasm");

// const EXTRA_BYTES: usize = 10000;
// const GAS: Gas = 50_000_000_000_000;

const TGAS: u64 = 1_000_000_000_000;

type TokenId = String;



pub fn is_valid_token_id(token_id: &TokenId) -> bool {
    for c in token_id.as_bytes() {
        match c {
            b'0'..=b'9' | b'a'..=b'z' => (),
            _ => return false,
        }
    }
    true
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtBalanceOfArgs {
    account_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtTransferArgs {
    receiver_id: AccountId,
    amount: U128,
    memo: Option<String>,
}


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Tokens,
    StorageDeposits,
    TokensByPosition,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenFactory {
    pub tokens: UnorderedMap<TokenId, TokenArgs>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub storage_balance_cost: Balance,
    pub tokens_by_position: UnorderedMap<String, TokenArgs>,
    pub ft_contract_id: AccountId,
    pub nft_contract_id: AccountId,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenArgs {
    owner_id: AccountId,
    // metadata: NFTContractMetadata,
    token_metadata: TokenMetadata,
    x: String, 
    y: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTMint {
    token_id: TokenId,
    receiver_id: AccountId,
    token_metadata: TokenMetadata,
}

// ---------| Token Creation Args |----------
// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct TokenCreationArgs {
//     // Define aquí las propiedades necesarias para crear el token, por ejemplo:
//     args: TokenArgs,
//     token_metadata: TokenMetadata,
// }


#[near_bindgen]
impl TokenFactory {
    #[init]
    pub fn new(ft_contract_id: AccountId, nft_contract_id: AccountId) -> Self {
        let mut storage_deposits = LookupMap::new(StorageKey::StorageDeposits);

        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = "a".repeat(64);
        storage_deposits.insert(&tmp_account_id, &0);
        let storage_balance_cost =
            Balance::from(env::storage_usage() - initial_storage_usage) * STORAGE_PRICE_PER_BYTE;
        storage_deposits.remove(&tmp_account_id);

        Self {
            tokens: UnorderedMap::new(StorageKey::Tokens),
            storage_deposits,
            storage_balance_cost,
            tokens_by_position: UnorderedMap::new(StorageKey::TokensByPosition),
            ft_contract_id,
            nft_contract_id,
        }
    }

    // fn get_min_attached_balance(&self, args: &TokenArgs) -> u128 {
    //     ((NFT_WASM_CODE.len() + EXTRA_BYTES + args.try_to_vec().unwrap().len() * 2) as Balance
    //         * STORAGE_PRICE_PER_BYTE)
    //         .into()
    // }

    // pub fn get_required_deposit(&self, args: TokenArgs, account_id: ValidAccountId) -> U128 {
    //     let args_deposit = self.get_min_attached_balance(&args);
    //     if let Some(previous_balance) = self.storage_deposits.get(account_id.as_ref()) {
    //         args_deposit.saturating_sub(previous_balance).into()
    //     } else {
    //         (self.storage_balance_cost + args_deposit).into()
    //     }
    // }

    #[payable]
    pub fn storage_deposit(&mut self) {
        let account_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        if let Some(previous_balance) = self.storage_deposits.get(&account_id) {
            self.storage_deposits
                .insert(&account_id, &(previous_balance + deposit));
        } else {
            assert!(deposit >= self.storage_balance_cost, "Deposit is too low");
            self.storage_deposits
                .insert(&account_id, &(deposit - self.storage_balance_cost));
        }
    }

    pub fn get_number_of_tokens(&self) -> u64 {
        self.tokens.len()
    }

    pub fn get_tokens(&self, from_index: u64, limit: u64) -> Vec<TokenArgs> {
        let tokens = self.tokens.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, tokens.len()))
            .filter_map(|index| tokens.get(index))
            .collect()
    }

    pub fn get_token(&self, token_id: TokenId) -> Option<TokenArgs> {
        self.tokens.get(&token_id)
    }

    #[payable]
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> U128 {
        const TOKEN_CREATION_FEE: u128 = 100;
        let required_fee = TOKEN_CREATION_FEE;
        
        // Asegurar que la cantidad transferida sea suficiente.
        assert!(amount.0 >= required_fee, "Insufficient FT tokens sent for token creation!");

        // Decodificar el mensaje para extraer argumentos para crear el token
        let nft_creation_args: TokenArgs = serde_json::from_str(&msg)
            .expect("Invalid message provided");

        // Crear el token basado en los argumentos
        self.internal_create_token(nft_creation_args);

        // Si hay más tokens de los necesarios, devolvemos el exceso al remitente.
        if amount.0 > required_fee {
            let refund_amount = amount.0 - required_fee;
            Promise::new(sender_id).function_call(
                b"ft_transfer".to_vec(),
                serde_json::to_vec(&FtTransferArgs {
                    receiver_id: env::predecessor_account_id(),
                    amount: refund_amount.into(),
                    memo: Some("Refund for excess deposit".to_string()),
                })
                .unwrap(),
                1,
                TGAS * 10,
            );
        }

        // Retornar la cantidad utilizada (que es la tarifa)
        U128(0)
    }
    

    // fn internal_create_token(&mut self, data: TokenCreationArgs) {
    fn internal_create_token(&mut self, data: TokenArgs) {
        let args = data;
        // let mut args = data.args;
        let token_metadata = args.token_metadata.clone();
    
        // Si hay un depósito adjunto, procesarlo.
        if env::attached_deposit() > 0 {
            self.storage_deposit();
        }
    
        // args.metadata.assert_valid();
        let number = self.get_number_of_tokens() + 1;
        // args.metadata.symbol = format!("R{}", number);
        let token_id = format!("R{}", number);
        // let token_id = args.metadata.symbol.to_ascii_lowercase();
        // assert!(is_valid_token_id(&token_id), "Invalid Symbol");
        let token_account_id_2 = self.nft_contract_id.clone(); // format!("{}.{}", token_id, env::current_account_id());
    
        let account_id = args.owner_id.clone();
        // args.metadata.name = format!("Realand #{}", number.to_string());
    
        // let initial_storage_usage = env::storage_usage();
    
        // Insert the token and make sure that the token doesn't exist
        assert!(
            self.tokens.insert(&token_id, &args).is_none(),
            "Token ID is already taken"
        );
    
        let position = format!("{}-{}", args.x, args.y);
        // Insert the position and make sure that the position doesn't exist
        assert!(
            self.tokens_by_position.insert(&position, &args).is_none(),
            "Position already exists"
        );
    
        //args.owner_id = env::current_account_id();
    
        let nft_token: NFTMint = NFTMint {
            token_id: token_id,
            receiver_id: account_id,
            token_metadata: TokenMetadata {
                title: Some(format!("Land #{}-{}", args.x, args.y)),
                description: token_metadata.description,
                media: token_metadata.media,
                media_hash: token_metadata.media_hash,
                copies: Some(1),
                issued_at: token_metadata.issued_at,
                expires_at: token_metadata.expires_at,
                starts_at: token_metadata.starts_at,
                updated_at: token_metadata.updated_at,
                extra: Some(format!("{{ 'x': {}, 'y': {} }}", args.x, args.y)),
                reference: token_metadata.reference,
                reference_hash: token_metadata.reference_hash,
            }
        };
    
        Promise::new(token_account_id_2.to_string())
            .function_call(b"nft_mint".to_vec(), serde_json::to_vec(&nft_token).unwrap(), 10710000000000000000000, TGAS * 50);
    }
    
    // --------------------------------------------------------------------------------------------
}
