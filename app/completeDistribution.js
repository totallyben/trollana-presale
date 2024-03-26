import dotenv from 'dotenv';
dotenv.config();

import bs58 from 'bs58';

import { Connection, PublicKey, Keypair, clusterApiUrl } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import {  TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';

import idl from '../target/idl/presale.json' assert { type: 'json' };

// Load the keypair from the local wallet JSON file asynchronously
async function loadKeypair(wallet) {
  const keypairData = await import(wallet, {
    assert: { type: 'json' },
  });
  return keypairData.default;
}

function getProgram(provider) {
  console.log(process.env.PRESALE_PROGRAM_ID);
  const programId = new PublicKey(process.env.PRESALE_PROGRAM_ID);
  return new Program(idl, programId, provider);
}

async function main() {

  const network = process.env.SOLANA_NETWORK || 'Mainnet';
  const localnetUrl = process.env.SOLANA_LOCALNET_URL;
  let rpcUrl = '';

  if (network != 'Localnet') {
    const selectedNetwork = WalletAdapterNetwork[network];
    rpcUrl = clusterApiUrl(selectedNetwork);
  } else {
    rpcUrl = localnetUrl || 'http://127.0.0.1:8899';
  }

  console.log(rpcUrl);
  const connection = new Connection(rpcUrl);
  
  // Assuming you have a keypair for the wallet (this is just an example)
  const ownerKeypairData = await loadKeypair(process.env.ANCHOR_WALLET);
  const ownerKeypair = Keypair.fromSecretKey(new Uint8Array(ownerKeypairData));

  // Set up the provider
  const ownerWallet = new Wallet(ownerKeypair);
  const provider = new AnchorProvider(connection, ownerWallet, AnchorProvider.defaultOptions());

  const program = getProgram(provider);

  const decodedBytes = bs58.decode(process.env.TOKEN_MINT_ADDRESS);

  const firstTenBytes = decodedBytes.slice(0, 10);
  const presaleRef = bs58.encode(firstTenBytes);
  
  const [presaleAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('presale_account')],
    program.programId
  );
  
  const tx = await program.methods
    .completeDist()
    .accounts({
      owner: ownerKeypair.publicKey,
      presaleAccount: presaleAccountPublicKey,
    })
    .rpc();

  console.log('Completed distribution');
  console.log("Your transaction signature", tx);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
