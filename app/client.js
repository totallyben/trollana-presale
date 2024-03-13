import dotenv from 'dotenv';
dotenv.config();

import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';

import idl from '../target/idl/presale.json' assert { type: 'json' };

// Load the keypair from the local wallet JSON file asynchronously
async function loadKeypair() {
  const keypairData = await import(process.env.ANCHOR_WALLET, {
    assert: { type: 'json' },
  });
  return keypairData.default;
}

async function main() {
  // Set up the connection to the Solana cluster
  const connection = new Connection(process.env.ANCHOR_PROVIDER_URL);

  // Assuming you have a keypair for the wallet (this is just an example)
  const keypairData = await loadKeypair();
  const walletKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
  // const wallet = new AnchorProvider.Wallet(walletKeypair);

  // Set up the provider
  const wallet = new Wallet(walletKeypair);
  const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());

  // Set the program ID to the public key of your deployed Anchor program
  const programId = new PublicKey(process.env.PRESALE_PROGRAM_ID);

  // Initialize the program
  const program = new Program(idl, programId, provider);

  // Define start and end time for the presale
  const { BN } = anchor.default;

  const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
  const endTime = startTime + 7 * 24 * 60 * 60; // One week from the start time

  const presaleToken = 'Shill';
  const presaleSymbol = 'SHILL';

  const [presaleAccountPublicKey, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from(presaleToken), Buffer.from(presaleSymbol)],
    program.programId
  );

  let accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);
  console.log('accountInfo', accountInfo);

  if (accountInfo) {
    const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);
    if (presaleAccountData.isInitialized) {
      console.log('presaleAccountData.isInitialized', presaleAccountData.isInitialized);
      return;
    }
  }

  const tx = await program.methods
    .initialize(presaleToken, presaleSymbol, bump, new BN(startTime), new BN(endTime))
    .accounts({
      presaleAccount: presaleAccountPublicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers(walletKeypair)
    .rpc();

  console.log('Presale initialized successfully');
  console.log("Your transaction signature", tx);

  accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);
  console.log('accountInfo', accountInfo);
  return;
  const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);

  console.log('presaleAccountData.isInitialized', presaleAccountData.isInitialized);
  console.log('presaleAccountData.isActive', presaleAccountData.isActive);

}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
