import dotenv from 'dotenv';
dotenv.config();

import { Connection, PublicKey, Keypair, Transaction } from '@solana/web3.js';
import { TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, AccountLayout, createAssociatedTokenAccountInstruction, getAssociatedTokenAddressSync  } from '@solana/spl-token';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';

import idl from '../target/idl/presale.json' assert { type: 'json' };

const presaleToken = 'Shill';
const presaleSymbol = 'SHILL';



console.log('TOKEN_2022_PROGRAM_ID', TOKEN_2022_PROGRAM_ID);
console.log('ASSOCIATED_TOKEN_PROGRAM_ID', ASSOCIATED_TOKEN_PROGRAM_ID);

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
  const connection = new Connection(process.env.ANCHOR_PROVIDER_URL);

  const buyerWalletKeypairData = await loadKeypair(process.env.PRESALE_BUYER_WALLET);
  const buyerWalletKeyPair = Keypair.fromSecretKey(new Uint8Array(buyerWalletKeypairData));

  // Set up the provider
  const buyerWallet = new Wallet(buyerWalletKeyPair);
  const provider = new AnchorProvider(connection, buyerWallet, AnchorProvider.defaultOptions());

  const program = getProgram(provider);

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

  const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);

  const mint = new PublicKey(process.env.TOKEN_MINT_ADDRESS);

  const buyerTokenAccountAddress = getAssociatedTokenAddressSync(
    mint, 
    buyerWallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const accountInfo = await connection.getAccountInfo(buyerTokenAccountAddress);
  if (!accountInfo) {
    console.log('Creating associated token account for the buyer...');
    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        buyerWallet.publicKey, // Payer of the transaction
        buyerTokenAccountAddress, // The associated token account address that will be created
        buyerWallet.publicKey, // The owner of the new associated token account
        mint, // Mint address for the token
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
    );

    // Sign and send the transaction to create the associated token accounts
    await provider.sendAndConfirm(transaction, [buyerWalletKeyPair]);
  }

  const { BN } = anchor.default;
  const solAmountLamports = new BN(anchor.web3.LAMPORTS_PER_SOL * 1); // Example: Buying tokens worth 1 SOL

  // Call the 'buy_tokens' method from the Anchor program
  const tx = await program.methods
    .buyTokens(presaleToken, presaleSymbol, solAmountLamports)
    .accounts({
      presaleAccount: presaleAccountPublicKey, 
      buyer: buyerWallet.publicKey,
      buyerTokenAccount: buyerTokenAccountAddress,
      tokenAccount: tokenAccountPublicKey,
      tokenAccountAuthority: tokenAuthorityPublicKey,
      destinationWallet: presaleAccountData.destinationWalletPubkey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      mint: mint,
    })
    .signers(buyerWalletKeyPair)
    .rpc();

  console.log('Tokens bought successfully. Transaction signature:', tx);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
