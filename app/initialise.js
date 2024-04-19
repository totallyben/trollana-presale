import dotenv from 'dotenv';
dotenv.config();

import bs58 from 'bs58';

import { Connection, PublicKey, Keypair, clusterApiUrl } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import {  TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';
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
  return new Program(idl, provider);
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

  const tokensPerSol = 1000000;
  const minBuy = 0.3;
  const maxBuy = 5;
  const presaleTokensAvailable = 300000000;
  const feePercent = 3;

  // Define start and end time for the presale
  const { BN } = anchor.default;

  const startDate = new Date("2024-04-02T18:00:00Z");
  const endDate = new Date("2024-04-09T17:59:59Z");
  const startTime = Math.floor(startDate / 1000); 
  const endTime = Math.floor(endDate / 1000);

  const decodedBytes = bs58.decode(process.env.TOKEN_MINT_ADDRESS);

  const firstTenBytes = decodedBytes.slice(0, 10);
  const presaleRef = bs58.encode(firstTenBytes); 

  console.log('presaleRef', presaleRef);
  // return;
  
  const [presaleAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleRef), Buffer.from('presale_account')],
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

  let accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);

  if (accountInfo) {
    const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);
    if (presaleAccountData.isInitialized) {
      console.log('presaleAccountData.isInitialized', presaleAccountData.isInitialized);
      console.log('presaleAccountData.destinationWalletPubkey', presaleAccountData.destinationWalletPubkey.toString());
      console.log('tokenAccountPublicKey', tokenAccountPublicKey.toString());
      return;
    }
  }

  // const destinationWalletKeypairData = await loadKeypair(process.env.PRESALE_RECIPIENT_WALLET);
  // const destinationWallet = Keypair.fromSecretKey(new Uint8Array(destinationWalletKeypairData));

  const recipientWallet = new PublicKey(process.env.PRESALE_RECIPIENT_WALLET_ADDRESS);
  const mint = new PublicKey(process.env.TOKEN_MINT_ADDRESS);
  
  const tx = await program.methods
    .initialize(presaleRef, new BN(startTime), new BN(endTime), tokensPerSol, feePercent, minBuy, maxBuy, new BN(presaleTokensAvailable))
    .accounts({
      presaleAccount: presaleAccountPublicKey,
      proceedsVault: proceedsVaultPublicKey,
      // user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      tokenAccount: tokenAccountPublicKey,
      tokenAccountAuthority: tokenAuthorityPublicKey,
      recipientWallet: recipientWallet,
      mint: mint,
    })
    .signers(ownerKeypair)
    .rpc();

  console.log('Presale initialized successfully');
  console.log("Your transaction signature", tx);
  
  // Start polling to check if the initialization is complete
  await pollForInitialization(program, presaleAccountPublicKey, tokenAccountPublicKey);
}

async function pollForInitialization(program, presaleAccountPublicKey, tokenAccountPublicKey) {
  const maxAttempts = 30; // Maximum number of attempts
  const interval = 2000; // Poll every 2000 milliseconds (2 seconds)

  let attempts = 0;

  return new Promise((resolve, reject) => {
    const intervalId = setInterval(async () => {
      if (attempts >= maxAttempts) {
        clearInterval(intervalId);
        reject(new Error("Polling for initialization timed out"));
        return;
      }

      try {
        let accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);
        if (accountInfo) {
          const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);
          if (presaleAccountData.isInitialized) {
            clearInterval(intervalId);
            console.log('Initialization confirmed');
            console.log('tokenAccountPublicKey', tokenAccountPublicKey.toString());
            resolve();
          }
        }
      } catch (error) {
        clearInterval(intervalId);
        reject(error);
      }

      attempts++;
    }, interval);
  });
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
