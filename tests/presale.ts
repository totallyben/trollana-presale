import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Presale } from "../target/types/presale";
import {
  PublicKey,
  Keypair,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Connection,
} from "@solana/web3.js";
import { assert } from "chai";

// const PRESALE_WALLET = new PublicKey(
//   "XrZEi1mXJeqaFAFbSbGpvr59rrEzth1dXedDE642ga5"
// );

describe("presale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Presale as Program<Presale>;
  const provider = anchor.AnchorProvider.env();

  it("Is initialized!", async () => {
    // Create an account to hold the presale data
    const presaleAccount = anchor.web3.Keypair.generate();

    // Define start and end times for the presale. For testing, you might use the current time for the start and a future time for the end.
    const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
    const endTime = startTime + 7 * 24 * 60 * 60; // One week from the start time

    // Create the transaction to initialize the presale
    const tx = await program.methods.initialize(new anchor.BN(startTime), new anchor.BN(endTime))
      .accounts({
        presaleAccount: presaleAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([presaleAccount])
      .rpc();
    
    console.log("Your transaction signature", tx);

    // Fetch the newly created presale account from the chain
    const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccount.publicKey);

    // Add assertions to verify the presale account's data
    assert.ok(presaleAccountData.startTime.eq(new anchor.BN(startTime)));
    assert.ok(presaleAccountData.endTime.eq(new anchor.BN(endTime)));
    assert.ok(presaleAccountData.isActive === true);
  });
});
