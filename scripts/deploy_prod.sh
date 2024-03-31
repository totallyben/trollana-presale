#!/usr/bin/env bash

display_usage () {
  echo
  echo "Usage:"
  echo "./deploy_prod.sh <toke-mint-address>"
  exit
}

solana config set -u m

ANCHOR_WORKSPACE=/home/gritzb/workspace/solana/trollana-presale
ANCHOR_DOT_ENV=${ANCHOR_WORKSPACE}/.env
ANCHOR_TOML=${ANCHOR_WORKSPACE}/Anchor.toml
DAPP_WORKSPACE=/home/gritzb/workspace/trollana/trollana-dapp

update_dot_envs () {
  sed -i "s/^SOLANA_NETWORK=.*/SOLANA_NETWORK=Mainnet/" ${ANCHOR_DOT_ENV}
  sed -i "s/^PRESALE_RECIPIENT_WALLET_ADDRESS=.*/PRESALE_RECIPIENT_WALLET_ADDRESS=${RECIPIENT_WALLET}/" ${ANCHOR_DOT_ENV}
  sed -i "s/^TOKEN_MINT_ADDRESS=.*/TOKEN_MINT_ADDRESS=${TOKEN_MINT_ADDRESS}/" ${ANCHOR_DOT_ENV}
}

update_anchor_toml () {
  sed -i "s/^cluster = .*/cluster = \"mainnet\"/" ${ANCHOR_TOML}
}

# program reset and deploy
deploy () {
  update_dot_envs
  update_anchor_toml
  
  cd ${ANCHOR_WORKSPACE}
  anchor build
  anchor deploy
  rsync -avz ${ANCHOR_WORKSPACE}/target/types/presale.ts ${DAPP_WORKSPACE}/idl/presale.ts
}

deploy