#!/usr/bin/env bash

REDEPLOY=$1

solana config set -u d

WALLET_DIR=~/workspace/solana/wallets
RECIPIENT_WALLET=EamZQNN5oyQZ48byFgU8cvYf3ma6SE3oBB623uWCCghY
ANCHOR_WORKSPACE=/home/gritzb/workspace/solana/trollana-presale
ANCHOR_DOT_ENV=${ANCHOR_WORKSPACE}/.env
ANCHOR_TOML=${ANCHOR_WORKSPACE}/Anchor.toml
DAPP_WORKSPACE=/home/gritzb/workspace/trollana/trollana-dapp

generate_token () {
  output=$(spl-token -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-metadata 2>&1)

  TOKEN_MINT_ADDRESS=$(echo "$output" | grep -oP 'Creating token \K[^\s]+' )
  echo ${output}
  echo "Token Address: ${TOKEN_MINT_ADDRESS}"
}

update_dot_envs() {
  sed -i "s/^SOLANA_NETWORK=.*/SOLANA_NETWORK=Devnet/" ${ANCHOR_DOT_ENV}
  sed -i "s/^PRESALE_RECIPIENT_WALLET_ADDRESS=.*/PRESALE_RECIPIENT_WALLET_ADDRESS=${RECIPIENT_WALLET}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^TOKEN_MINT_ADDRESS=.*/TOKEN_MINT_ADDRESS=${TOKEN_MINT_ADDRESS}/" ${ANCHOR_DOT_ENV}
}

update_anchor_toml() {
  sed -i "s/^cluster = .*/cluster = \"devnet\"/" ${ANCHOR_TOML}
}

mint_tokens () {
  TOKEN_NAME=Banana
  SYMBOL=BNN
  URL="example.com"
  NUM_TOKENS=1000000000

  # initialise the meta data
  spl-token initialize-metadata -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ${TOKEN_MINT_ADDRESS} ${TOKEN_NAME} ${SYMBOL} ${URL}

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
}

# program reset and deploy
reset () {
  generate_token
  update_dot_envs
  update_anchor_toml
  
  cd ${ANCHOR_WORKSPACE}
  # anchor build
  # anchor deploy
  rsync -avz ${ANCHOR_WORKSPACE}/target/types/presale.ts ${DAPP_WORKSPACE}/idl/presale.ts

  echo "Initialising..."
  output="$(node app/initialise.js)" 
  tokenAccountPublicKey=$(echo "$output" | grep 'tokenAccountPublicKey' | awk '{print $2}')
  echo "tokenAccountPublicKey ${tokenAccountPublicKey}"

  sed -i "s/^TOKEN_ACCOUNT_PUBLIC_KEY=.*/TOKEN_ACCOUNT_PUBLIC_KEY=${tokenAccountPublicKey}/" ${ANCHOR_DOT_ENV}

  . ${ANCHOR_DOT_ENV}
  mint_tokens
  send_presale_tokens
  
  echo "TOKEN MINT: ${TOKEN_MINT_ADDRESS}"
}

reset