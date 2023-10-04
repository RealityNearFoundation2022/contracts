# Factory NFT Contract


## Build


````bash

# deploy the NFT contract

# este nft solo debería dejar mintear al factory
near deploy --accountId $NFT --wasmFile src/nft-contract/nft_simple.wasm --initFunction 'new' --initArgs '{}'


near call $NFT new '{ "owner_id": "'$FACTORY'",
         "metadata": {
            "spec": "nft-1.0.0",
            "name": "Reality Lands",
            "symbol": "RLTSLANDS",
            "icon": "data:image/svg+xml,%3C…",
            "reference": "https://example.com/wbtc.json",
            "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
         }}' --accountId $NFT


# deploy the NFT factory
near deploy --accountId $FACTORY --wasmFile target/wasm32-unknown-unknown/release/nft_factory.wasm --initFunction 'new' --initArgs '{}'

# near call $FACTORY new  --accountId guxal.testnet

near call $FACTORY new '{ "ft_contract_id": "'$FT'", "nft_contract_id": "'$NFT'" }' --accountId $FACTORY

````

## Get required Deposit

````bash
near view $ID get_required_deposit '{
      "args": {
         "owner_id": "guxal.testnet",
         "metadata": {
            "spec": "nft-1.0.0",
            "name": "Wrapped Bitcoin",
            "symbol": "WBTC5",
            "icon": "data:image/svg+xml,%3C…",
            "reference": "https://example.com/wbtc.json",
            "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
         },
         "x": "1",
         "y": "2"
      },
      "account_id": "guxal.testnet"
   }'
````

## Create Token NFT
````bash
near call $CONTRACT_FACTORY create_token_pre '{
      "args": {
      "owner_id": "guxal.testnet",
      "metadata": {
         "spec": "nft-1.0.0",
         "name": "Wrapped Bitcoin",
         "symbol": "WBTC5",
         "icon": "data:image/svg+xml,%3C…",
         "reference": "https://example.com/wbtc.json",
         "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
      },
      "x": "1",
      "y": "20"
    },
   "token_metadata": {
         "description": "comuna 16",
         "media": "image",
         "media_hash": "hash"
   },
   "ft_amount": 100
}' --accountId test2221.testnet --depositYocto 3331110000000000000000000 --gas 300000000000000
````



near call $ID create_token '{"args": {"owner_id": "yudcumba.testnet","metadata": {
      "spec": "nft-1.0.0",
      "name": "#2426",
      "symbol": "#R2426",
      "icon": "data:image/svg+xml,%3C…",
      "reference": "",
      "reference_hash": ""
      },
      "x": "24",
      "y": "26"
   }
}' --accountId guxal.testnet --depositYocto 3329810000000000000000000 --gas 300000000000000

    
## Get Data About Tokens Created

````bash
near view $ID get_tokens '{"from_index": 0, "limit": 10 }'
near view $ID get_number_of_tokens 
````


```bash

near call $FT ft_transfer_call "{   \"receiver_id\":\"$FACTORY\",   \"amount\":\"100\",   \"msg\":\"{ \\\"args\\\": { \\\"owner_id\\\": \\\"guxal.testnet\\\", \\\"metadata\\\": { \\\"spec\\\": \\\"nft-1.0.0\\\", \\\"name\\\": \\\"Wrapped Bitcoin\\\", \\\"symbol\\\": \\\"WBTC5\\\", \\\"icon\\\": \\\"data:image/svg+xml,%3C…\\\", \\\"reference\\\": \\\"https://example.com/wbtc.json\\\", \\\"reference_hash\\\": \\\"AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M=\\\" }, \\\"x\\\": \\\"43\\\", \\\"y\\\": \\\"20\\\"},\\\"token_metadata\\\": {    \\\"description\\\": \\\"comuna 16\\\",    \\\"media\\\": \\\"image\\\",    \\\"media_hash\\\": \\\"hash\\\"} }\"}" --accountId test2221.testnet --depositYocto 1 --gas 300000000000000



near call $FT ft_transfer_call "{ \"receiver_id\":\"$FACTORY\",   \"amount\":\"100\",   \"msg\":\"{ \\\"owner_id\\\": \\\"test2221.testnet\\\", \\\"x\\\": \\\"43\\\", \\\"y\\\": \\\"27\\\" ,\\\"token_metadata\\\": {  \\\"description\\\": \\\"comuna 16\\\",    \\\"media\\\": \\\"image\\\",    \\\"media_hash\\\": \\\"hash\\\"} }\"}" --accountId test2221.testnet --depositYocto 1 --gas 300000000000000


```