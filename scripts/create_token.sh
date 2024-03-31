#!/usr/bin/env bash

RECIPIENT_WALLET_ADDRESS=$1

solana config set -u m

display_usage () {
  echo
  echo "Usage:"
  echo "./deploy_prod.sh <recipient-wallet-address>"
  exit
}

if [[ -z ${RECIPIENT_WALLET_ADDRESS} ]]; then
  echo "RECIPIENT_WALLET_ADDRESS cannot be empty"
  display_usage
fi

generate_token () {
  output=$(spl-token -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-metadata 2>&1)

  TOKEN_MINT_ADDRESS=$(echo "$output" | grep -oP 'Creating token \K[^\s]+' )
  echo ${output}
  echo "Token Address: ${TOKEN_MINT_ADDRESS}"
}

update_dot_envs () {
  sed -i "s/^PRESALE_RECIPIENT_WALLET_ADDRESS=.*/PRESALE_RECIPIENT_WALLET_ADDRESS=${RECIPIENT_WALLET_ADDRESS}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^TOKEN_MINT_ADDRESS=.*/TOKEN_MINT_ADDRESS=${TOKEN_MINT_ADDRESS}/" ${ANCHOR_DOT_ENV}
}

mint_tokens () {
  TOKEN_NAME="Minions on SOL"
  SYMBOL=MINISOL
  URL="https://minionsonsol.fun"
  NUM_TOKENS=1000000000

  # initialise the meta data
  spl-token initialize-metadata ${TOKEN_MINT_ADDRESS} "${TOKEN_NAME}" "${SYMBOL}" "${URL}"

  # mint tokens
  spl-token create-account ${TOKEN_MINT_ADDRESS}
  spl-token mint ${TOKEN_MINT_ADDRESS} ${NUM_TOKENS}

  # close off the mint
  spl-token authorize ${TOKEN_MINT_ADDRESS} mint --disable
}

# send_presale_tokens () {
#   PRESALE_TOKENS=100000000
#   TOKEN_ACCOUNT_ADDRESS=${TOKEN_ACCOUNT_PUBLIC_KEY}
#   spl-token transfer --fund-recipient ${TOKEN_MINT_ADDRESS} ${PRESALE_TOKENS} ${TOKEN_ACCOUNT_ADDRESS}
# }

# program reset and deploy
reset () {
  # generate_token
  # update_dot_envs
  
  echo "Initialising..."
  output="$(node app/initialise.js)" 
  tokenAccountPublicKey=$(echo "$output" | grep 'tokenAccountPublicKey' | awk '{print $2}')
  echo "tokenAccountPublicKey ${tokenAccountPublicKey}"

  sed -i "s/^TOKEN_ACCOUNT_PUBLIC_KEY=.*/TOKEN_ACCOUNT_PUBLIC_KEY=${tokenAccountPublicKey}/" ${ANCHOR_DOT_ENV}
  
  return
  . ${ANCHOR_DOT_ENV}
  mint_tokens
  # send_presale_tokens

  # solana airdrop 100 CaENpeu35q6rqn6mih7aTKUUsXQJf5VFCsPvJg9ZkrTu
  
  echo "TOKEN MINT: ${TOKEN_MINT_ADDRESS}"
}

reset