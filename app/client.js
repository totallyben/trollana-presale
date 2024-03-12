import dotenv from 'dotenv';
dotenv.config();

import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
const { Provider, SystemProgram } = anchor.web3;
// import type { Counter } from "../target/types/counter";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Presale;

async function main() {
    // Client
    console.log("My address:", program.provider.publicKey.toString());
    const balance = await program.provider.connection.getBalance(program.provider.publicKey);
    console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

    // const presaleAccount = new anchor.web3.PublicKey(process.env.PRESALE_ACCOUNT_ID);

    // Define start and end time for the presale
    const { BN } = anchor.default
    
    const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
    const endTime = startTime + 7 * 24 * 60 * 60; // One week from the start time

    const presaleAccount = anchor.web3.Keypair.generate();
    
    // Call the initialize function from your Solana program
    await program.methods.initialize(new BN(startTime), new BN(endTime))
    .accounts({
        presaleAccount: presaleAccount.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
    })
    .signers([presaleAccount])
    .rpc();

    console.log('Presale initialized successfully');
}

main().then(() => process.exit(0)).catch(error => {
    console.error(error);
    process.exit(1);
});