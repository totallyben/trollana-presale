#!/usr/bin/env bash

STEP_TO_RUN=$1

WALLET_DIR=~/workspace/solana/wallets
BUYER_WALLET=${WALLET_DIR}/presale-buyer.json
RECIPIENT_WALLET=${WALLET_DIR}/presale-recipient.json
TOKEN_WALLET=${WALLET_DIR}/presale-tokens.json
ANCHOR_WORKSPACE=/home/gritzb/workspace/solana/trollana-presale
ANCHOR_DOT_ENV=${ANCHOR_WORKSPACE}/.env

reset_wallets () {
  rm -rf ${BUYER_WALLET} ${RECIPIENT_WALLET}
  solana-keygen new --outfile ${BUYER_WALLET}
  solana-keygen new --outfile ${RECIPIENT_WALLET}
  solana-keygen new --outfile ${TOKEN_WALLET}
  solana airdrop 1000 ${RECIPIENT_WALLET}
  solana airdrop 1000 ${BUYER_WALLET}
  
}

generate_token () {
  output=$(spl-token -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-metadata 2>&1)

  TOKEN_MINT_ADDRESS=$(echo "$output" | grep -oP 'Creating token \K[^\s]+' )
  echo ${output}
  echo "Token Address: ${TOKEN_MINT_ADDRESS}"
}

update_dot_envs() {
  recipient_pubkey=$(solana-keygen pubkey ${RECIPIENT_WALLET})
  
  sed -i "s/^PRESALE_RECIPIENT_WALLET_ADDRESS=.*/PRESALE_RECIPIENT_WALLET_ADDRESS=${recipient_pubkey}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^TOKEN_MINT_ADDRESS=.*/TOKEN_MINT_ADDRESS=${TOKEN_MINT_ADDRESS}/" ${ANCHOR_DOT_ENV}
}

mint_tokens () {
  TOKEN_NAME=Banana
  SYMBOL=BNN
  URL="example.com"
  NUM_TOKENS=1000000000

  # initialise the meta data
  spl-token initialize-metadata ${TOKEN_MINT_ADDRESS} ${TOKEN_NAME} ${SYMBOL} ${URL}

  # mint tokens
  spl-token create-account ${TOKEN_MINT_ADDRESS}
  spl-token mint ${TOKEN_MINT_ADDRESS} ${NUM_TOKENS}

  # close off the mint
  spl-token authorize ${TOKEN_MINT_ADDRESS} mint --disable
}

send_presale_tokens () {
  PRESALE_TOKENS=100000000
  TOKEN_ACCOUNT_ADDRESS=${TOKEN_ACCOUNT_PUBLIC_KEY}
  spl-token transfer --fund-recipient ${TOKEN_MINT_ADDRESS} ${PRESALE_TOKENS} ${TOKEN_ACCOUNT_ADDRESS}
  spl-token transfer --fund-recipient --allow-unfunded-recipient ${TOKEN_MINT_ADDRESS} ${PRESALE_TOKENS} ${TOKEN_WALLET}
}

# program reset and deploy
step1 () {
  if [[ ${STEP_TO_RUN} != 1 ]]; then
    return
  fi
  reset_wallets
  generate_token
  update_dot_envs
  
  cd ${ANCHOR_WORKSPACE}
  anchor deploy

  echo "TOKEN MINT: ${TOKEN_MINT_ADDRESS}"
}

# program initialise
step2 () {
  if [[ ${STEP_TO_RUN} != 2 ]]; then
    return
  fi
  node app/initialise.js
}

# post program initialise
step3 () {
  if [[ ${STEP_TO_RUN} != 3 ]]; then
    return
  fi

  . ${ANCHOR_DOT_ENV}
  mint_tokens
  send_presale_tokens
  
  echo "TOKEN MINT: ${TOKEN_MINT_ADDRESS}"
}

step1
step2
step3
