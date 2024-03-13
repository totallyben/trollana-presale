# Testing

## Initialise wallets

```
WALLET_DIR=~/workspace/solana/wallets
BUYER_WALLET=${WALLET_DIR}/presale-buyer.json
RECIPIENT_WALLET=${WALLET_DIR}/presale-recipient.json

rm -rf ${BUYER_WALLET} ${RECIPIENT_WALLET}
solana-keygen new --outfile ${BUYER_WALLET}
solana-keygen new --outfile ${RECIPIENT_WALLET}
solana airdrop 1000 ${BUYER_WALLET}
```

## SPL Token

Create the spl token and get the token mint address:

```
spl-token -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-metadata
```

Prep the token:

```
TOKEN_MINT_ADDRESS=<token-mint-address>
TOKEN_NAME=<token-name>
SYMBOL=<token-symbol>
URL=<token-url>
NUM_TOKENS=1000000000

# initialise the meta data
spl-token initialize-metadata ${TOKEN_MINT_ADDRESS} ${TOKEN_NAME} ${SYMBOL} ${URL}

# mint tokens
spl-token create-account ${TOKEN_MINT_ADDRESS}
spl-token mint ${TOKEN_MINT_ADDRESS} ${NUM_TOKENS}

# close off the mint
spl-token authorize ${TOKEN_MINT_ADDRESS} mint --disable
```

Send the presale tokens:

```
PRESALE_TOKENS=100000000
TOKEN_ACCOUNT_ADDRESS=<token-account-address>

spl-token transfer --fund-recipient ${TOKEN_MINT_ADDRESS} ${PRESALE_TOKENS} ${TOKEN_ACCOUNT_ADDRESS}
```