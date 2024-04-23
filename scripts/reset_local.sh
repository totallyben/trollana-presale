#!/usr/bin/env bash

solana config set -u l

WALLET_DIR=~/workspace/solana/wallets
BUYER_WALLET=${WALLET_DIR}/presale-buyer.json
RECIPIENT_WALLET=${WALLET_DIR}/presale-recipient.json
FEE_WALLET=${WALLET_DIR}/presale-fee.json
ANCHOR_WORKSPACE=/home/gritzb/workspace/solana/trollana-presale
ANCHOR_DOT_ENV=${ANCHOR_WORKSPACE}/.env
ANCHOR_TOML=${ANCHOR_WORKSPACE}/Anchor.toml
DAPP_WORKSPACE=/home/gritzb/workspace/trollana/trollana-dapp

reset_wallets () {
  rm -rf ${BUYER_WALLET} ${RECIPIENT_WALLET}
  solana-keygen new --outfile ${BUYER_WALLET}
  solana-keygen new --outfile ${RECIPIENT_WALLET}
  solana-keygen new --outfile ${FEE_WALLET}
  solana airdrop 1000 ${RECIPIENT_WALLET}
  solana airdrop 1000 ${BUYER_WALLET}
  solana airdrop 1000 ${FEE_WALLET}
}

generate_token () {
  #output=$(spl-token -p TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-metadata 2>&1)
  output=$(spl-token -p TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA create-token 2>&1)

  TOKEN_MINT_ADDRESS=$(echo "$output" | grep -oP 'Creating token \K[^\s]+' )
  echo ${output}
  echo "Token Address: ${TOKEN_MINT_ADDRESS}"
}

update_dot_envs() {
  recipient_pubkey=$(solana-keygen pubkey ${RECIPIENT_WALLET})
  fee_pubkey=$(solana-keygen pubkey ${FEE_WALLET})
  
  sed -i "s/^SOLANA_NETWORK=.*/SOLANA_NETWORK=Localnet/" ${ANCHOR_DOT_ENV}
  sed -i "s/^PRESALE_RECIPIENT_WALLET_ADDRESS=.*/PRESALE_RECIPIENT_WALLET_ADDRESS=${recipient_pubkey}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^PRESALE_FEE_WALLET_ADDRESS=.*/PRESALE_FEE_WALLET_ADDRESS=${fee_pubkey}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^TOKEN_MINT_ADDRESS=.*/TOKEN_MINT_ADDRESS=${TOKEN_MINT_ADDRESS}/" ${ANCHOR_DOT_ENV}
}

update_anchor_toml() {
  sed -i "s/^cluster = .*/cluster = \"localnet\"/" ${ANCHOR_TOML}
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
}

# program reset and deploy
reset () {
  reset_wallets
  generate_token
  update_dot_envs
  update_anchor_toml
  
  cd ${ANCHOR_WORKSPACE}
  anchor build
  solana program deploy target/deploy/presale.so
  rsync -avz ${ANCHOR_WORKSPACE}/target/types/presale.ts ${DAPP_WORKSPACE}/idl/presale.ts
  rsync -avz ${ANCHOR_WORKSPACE}/target/types/presale.ts ${DAPP_WORKSPACE}/idl/presaleIdl.ts
  rsync -avz ${ANCHOR_WORKSPACE}/target/idl/presale.json ${DAPP_WORKSPACE}/idl/presale.json

  sed -i "s/export type Presale =/export const IDL: Presale =/" ${DAPP_WORKSPACE}/idl/presaleIdl.ts
  echo "" >> ${DAPP_WORKSPACE}/idl/presale.ts
  cat ${DAPP_WORKSPACE}/idl/presaleIdl.ts >> ${DAPP_WORKSPACE}/idl/presale.ts
  rm ${DAPP_WORKSPACE}/idl/presaleIdl.ts

  echo "Initialising..."
  output="$(node app/initialise.js)" 
  tokenAccountPublicKey=$(echo "$output" | grep 'tokenAccountPublicKey' | awk '{print $2}')
  echo "tokenAccountPublicKey ${tokenAccountPublicKey}"

  sed -i "s/^TOKEN_ACCOUNT_PUBLIC_KEY=.*/TOKEN_ACCOUNT_PUBLIC_KEY=${tokenAccountPublicKey}/" ${ANCHOR_DOT_ENV}
  
  . ${ANCHOR_DOT_ENV}
  mint_tokens
  send_presale_tokens

  solana airdrop 100 CaENpeu35q6rqn6mih7aTKUUsXQJf5VFCsPvJg9ZkrTu
  
  echo "TOKEN MINT: ${TOKEN_MINT_ADDRESS}"
}

reset