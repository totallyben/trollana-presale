import dotenv from 'dotenv';
dotenv.config();

import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import {  TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';

import idl from '../target/idl/presale.json' assert { type: 'json' };

const presaleToken = 'Shill';
const presaleSymbol = 'SHILL';

// Load the keypair from the local wallet JSON file asynchronously
async function loadKeypair(wallet) {
  const keypairData = await import(wallet, {
    assert: { type: 'json' },
  });
  return keypairData.default;
}

function getProgram(provider) {
  const programId = new PublicKey(process.env.PRESALE_PROGRAM_ID);
  return new Program(idl, programId, provider);
}

async function main() {
  // Set up the connection to the Solana cluster
  const connection = new Connection(process.env.ANCHOR_PROVIDER_URL);
  
  // Assuming you have a keypair for the wallet (this is just an example)
  const ownerKeypairData = await loadKeypair(process.env.ANCHOR_WALLET);
  const ownerKeypair = Keypair.fromSecretKey(new Uint8Array(ownerKeypairData));

  // Set up the provider
  const ownerWallet = new Wallet(ownerKeypair);
  const provider = new AnchorProvider(connection, ownerWallet, AnchorProvider.defaultOptions());

  const program = getProgram(provider);

  // Define start and end time for the presale
  const { BN } = anchor.default;

  const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
  const endTime = startTime + 7 * 24 * 60 * 60; // One week from the start time

  const [presaleAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleToken), Buffer.from(presaleSymbol)],
    program.programId
  );

  const [tokenAccountPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleToken), Buffer.from(presaleSymbol), 'token_account'],
    program.programId
  );

  const [tokenAuthorityPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleToken), Buffer.from(presaleSymbol), 'token_account_authority'],
    program.programId
  );

  let accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);

  if (accountInfo) {
    const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);
    if (presaleAccountData.isInitialized) {
      console.log('presaleAccountData.isInitialized', presaleAccountData.isInitialized);
      console.log('presaleAccountData.destinationWalletPubkey', presaleAccountData.destinationWalletPubkey);
      console.log('tokenAccountPublicKey', tokenAccountPublicKey);
      return;
    }
  }

  // const destinationWalletKeypairData = await loadKeypair(process.env.PRESALE_RECIPIENT_WALLET);
  // const destinationWallet = Keypair.fromSecretKey(new Uint8Array(destinationWalletKeypairData));

  const destinationWallet = new PublicKey(process.env.PRESALE_RECIPIENT_WALLET_ADDRESS);
  const mint = new PublicKey(process.env.TOKEN_MINT_ADDRESS);

  const tokensPerSol = 100000;

  const tx = await program.methods
    .initialize(presaleToken, presaleSymbol, new BN(startTime), new BN(endTime))
    .accounts({
      presaleAccount: presaleAccountPublicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      tokenAccount: tokenAccountPublicKey,
      tokenAccountAuthority: tokenAuthorityPublicKey,
      destinationWallet: destinationWallet,
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
            console.log('tokenAccountPublicKey', tokenAccountPublicKey);
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
