
export ID=token.guxal.testnet

// mil trillones 
near call token.guxal.testnet new '{"owner_id": "token.guxal.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Reality", "symbol": "REL", "decimals": 8 }}' --accountId token.guxal.testnet

near call tokenreality.guxal.testnet new '{"owner_id": "tokenreality.guxal.testnet", "total_supply": "10000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Reality", "symbol": "RLTS", "decimals": 8 }}' --accountId tokenreality.guxal.testnet

near call $ID ft_transfer '{"receiver_id": "'bob.guxal.testnet'", "amount": "19"}' --accountId $ID --amount 0.000000000000000000000001

near call $ID ft_transfer '{"receiver_id": "'guxal.testnet'", "amount": "100"}' --accountId $CONTRACT --amount 0.000000000000000000000001

near view $ID ft_balance_of '{"account_id": "bob.guxal.testnet"}'

near view $ID ft_balance_of '{"account_id": "token.guxal.testnet"}'

near call $ID ft_transfer '{"receiver_id": "'bob.guxal.testnet'", "amount": "19"}' --accountId token.guxal.testnet --amount 0.000000000000000000000001


near call $ID storage_deposit '{"account_id": "realitynearus1.testnet"}' --accountId guxal.testnet --amount 0.00125

near call $ID ft_transfer '{"receiver_id": "'1357iramuy_08.testnet'", "amount": "190000000000000"}' --accountId $ID --amount 0.000000000000000000000001

# ft

cargo generate --git https://github.com/near-examples/ft --name ft

# nft
cargo generate --git https://github.com/near-examples/nft --name nft


export ID2=nft.guxal.testnet


near call $ID2 nft_mint '{"token_id": "0", "receiver_id": "'$ID2'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId $ID2 --deposit 0.1