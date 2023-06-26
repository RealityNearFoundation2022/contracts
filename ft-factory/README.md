


near call dev-1671034789480-94459791041815 new  --accountId guxal.testnet



near view dev-1671034789480-94459791041815 get_required_deposit '{
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


near call dev-1671034789480-94459791041815 create_token '{
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
    }
}' --accountId guxal.testnet --depositYocto 2235130000000000000000000


    

near view dev-1671034789480-94459791041815 get_tokens '{"from_index": 0, "limit": 10 }'