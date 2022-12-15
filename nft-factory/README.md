


near call dev-1671111325362-71022326044644 new  --accountId guxal.testnet



near view dev-1671111325362-71022326044644 get_required_deposit '{
      "args": {
         "owner_id": "guxal.testnet",
         "metadata": {
            "spec": "nft-1.0.0",
            "name": "Wrapped Bitcoin",
            "symbol": "WBTC2",
            "icon": "data:image/svg+xml,%3C…",
            "reference": "https://example.com/wbtc.json",
            "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="
         }
         },
         "account_id": "guxal.testnet"
      }'


near call dev-1671111325362-71022326044644 create_token '{
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


    

near view dev-1671111325362-71022326044644 get_tokens '{"from_index": 0, "limit": 10 }'



near view dev-1671111325362-71022326044644 get_required_deposit '{
        "args": {
        "owner_id": "guxal.testnet",
        "total_supply": "100000000000000",
        "metadata": {
           "spec": "ft-1.0.0",
           "name": "Wrapped Bitcoin",
           "symbol": "WBTC2",
           "icon": "data:image/svg+xml,%3C…",
           "reference": "https://example.com/wbtc.json",
           "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M=",
           "decimals": 8
        }
      },
      "account_id": "guxal.testnet"
      }'




near call wbtc2.dev-1671111325362-71022326044644 nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId bob.guxal.testnet --deposit 0.1



near view dev-1671111325362-71022326044644 get_number_of_tokens 