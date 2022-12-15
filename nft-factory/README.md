# Factory NFT Contract


## Build


````bash

near deploy --accountId $ID --wasmFile target/wasm32-unknown-unknown/release/nft_factory.wasm --initFunction 'new' --initArgs '{}'

near call $ID new  --accountId guxal.testnet

````

## Get required Deposit

````bash
near view $ID get_required_deposit '{
      "args": {
         "owner_id": "$ID",
         "metadata": {
            "spec": "nft-1.0.0",
            "name": "Wrapped Bitcoin",
            "symbol": "WBTC5",
            "icon": "data:image/svg+xml,%3C…",
            "reference": "https://example.com/wbtc.json",
            "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
         }
         },
         "account_id": "guxal.testnet"
      }'
````

## Create Token NFT
````bash
near call $ID create_token '{
      "args": {
      "owner_id": "guxal.testnet",
      "metadata": {
         "spec": "nft-1.0.0",
         "name": "Wrapped Bitcoin",
         "symbol": "WBTC5",
         "icon": "data:image/svg+xml,%3C…",
         "reference": "https://example.com/wbtc.json",
         "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
      }
    }
}' --accountId guxal.testnet --depositYocto 3330910000000000000000000 --gas 300000000000000
````

    
## Get Data About Tokens Created

````bash
near view $ID get_tokens '{"from_index": 0, "limit": 10 }'
near view $ID get_number_of_tokens 
````
