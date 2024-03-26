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

function refFromKey(key) {
  if (!key || key === undefined) {
    return '';
  }
  console.log('key', key);
  const decodedBytes = bs58.decode(key);
  const firstTenBytes = decodedBytes.slice(0, 10);
  return bs58.encode(firstTenBytes);
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


  const buyerRef = refFromKey(ownerKeypair.publicKey.toString());
  console.log('presaleRef', presaleRef);
  console.log('buyerRef', buyerRef);
  // return;
  
  const [presaleAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('presale_account')],
    program.programId
  );
    
  const [buyerAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from(buyerRef), Buffer.from('buyer_account')],
    program.programId
  );
  
  const [tokenAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('token_account')],
    program.programId
  );
  
  const [tokenAuthorityPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('token_account_authority')],
    program.programId
  );
  
  const [proceedsVaultPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('proceeds_vault')],
    program.programId
  );

  const destinationWallet = new PublicKey(process.env.PRESALE_RECIPIENT_WALLET_ADDRESS);
  const mint = new PublicKey(process.env.TOKEN_MINT_ADDRESS);

  const destinationWalletTokenAccount = getAssociatedTokenAddressSync(
    mint, 
    destinationWallet,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);

  console.log(presaleAccountData.num_sales);

  const numSales = presaleAccountData.num_sales || 0;
  console.log('presaleAccountData.numSales', presaleAccountData.numSales);
  console.log('numSales', numSales);

  // Convert num_sales (assuming it's a u32) to a byte array in little-endian format
  const numSalesBuffer = Buffer.alloc(4); // 4 bytes for u32
  numSalesBuffer.writeUInt32LE(presaleAccountData.numSales, 0);

  const [purchaseReceiptPublicKey] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(presaleRef),
      numSalesBuffer,
      Buffer.from('purchase_receipt'),
    ],
    program.programId
  );

  const { BN } = anchor.default;
  const solAmountLamports = new BN(anchor.web3.LAMPORTS_PER_SOL * Number(1.5));

  const tx = await program.methods
    .buyTokens(presaleRef, buyerRef, solAmountLamports)
    .accounts({
      presaleAccount: presaleAccountPublicKey,
      buyer: ownerKeypair.publicKey,
      buyerAccount: buyerAccountPublicKey,
      proceedsVault: proceedsVaultPublicKey,
      purchaseReceipt: purchaseReceiptPublicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log('Presale ended successfully');
  console.log("Your transaction signature", tx);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
