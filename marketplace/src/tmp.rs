//place an offer on a specific sale. The sale will go through as long as your deposit is greater than or equal to the list price
#[payable]
pub fn offer(&mut self, nft_contract_id: AccountId, token_id: String, ft_amount: U128) {
    // Verify FT amount is greater than 0
    assert!(ft_amount > 0, "Attached deposit must be greater than 0");

    //convert the nft_contract_id from a AccountId to an AccountId
    let contract_id: AccountId = nft_contract_id.into();
    //get the unique sale ID (contract + DELIMITER + token ID)
    let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
    
    //get the sale object from the unique sale ID. If the sale doesn't exist, panic.
    let sale = self.sales.get(&contract_and_token_id).expect("No sale");
    
    //get the buyer ID which is the person who called the function and make sure they're not the owner of the sale
    let buyer_id = env::predecessor_account_id();
    assert_ne!(sale.owner_id, buyer_id, "Cannot bid on your own sale.");
    
    //get the u128 price of the token (dot 0 converts from U128 to u128)
    let price = sale.sale_conditions.0;

    //make sure the FT amount is greater than the price
    assert!(ft_amount >= price, "Attached deposit must be greater than or equal to the current price: {:?}", price);

    //process the purchase (which will remove the sale, transfer and get the payout from the nft contract, and then distribute royalties) 
    self.process_purchase(
        contract_id,
        token_id,
        ft_amount,
        buyer_id,
    );
}

#[private]
pub fn process_purchase(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    price: U128,
    buyer_id: AccountId,
) -> Promise {
    //get the sale object by removing the sale
    let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

    // Transfer FTs to this contract
    ext_ft::ft_transfer(
        buyer_id.clone(),
        price,
        None,
        &nft_contract_id,
        1,
        GAS_FOR_FT_TRANSFER,
    )
    // Handle FT transfer callback
    .then(ext_self::after_ft_transfer(
        buyer_id,
        price,
        token_id,
        sale.approval_id,
        &env::current_account_id(),
        NO_DEPOSIT,
        GAS_FOR_AFTER_FT_TRANSFER,
    ))
}

#[private]
pub fn after_ft_transfer(
    &mut self,
    buyer_id: AccountId,
    amount: U128,
    token_id: String,
    approval_id: U64,
    nft_contract_id: &AccountId,
) -> Promise {
    // Verify that the FT transfer was successful
    assert_success();

    // Do the same as in your `nft_transfer_payout` method
    ext_contract::nft_transfer_payout(
        buyer_id,
        token_id,
        approval_id,
        "payout from market".to_string(),
        amount,
        10,
        nft_contract_id.clone(),
        1,
        GAS_FOR_NFT_TRANSFER,
    )
    .then(ext_self::resolve_purchase(
        buyer_id,
        amount,
        &env::current_account_id(),
        NO_DEPOSIT,
        GAS_FOR_ROYALTIES,
    ))
}

// other methods unchanged
