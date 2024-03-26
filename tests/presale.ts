import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
//#import { Presale } from "../target/types/presale";
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
    // Define start and end times for the presale. For testing, you might use the current time for the start and a future time for the end.
    const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
    const endTime = startTime + 7 * 24 * 60 * 60; // One week from the start time

    const presaleToken = "Shill";
    const presaleSymbol = "SHILL";

    const [presaleAccountPublicKey, bump] = PublicKey.findProgramAddressSync(
      [
          Buffer.from(presaleToken),
          Buffer.from(presaleSymbol),
      ],
      program.programId
    );

    const walletKeypair = provider.wallet.payer;

    const accountInfo = await program.provider.connection.getAccountInfo(presaleAccountPublicKey);

    if (accountInfo) {
      const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);
      if (presaleAccountData.isInitialized) {
        console.log('presaleAccountData.isInitialized', presaleAccountData.isInitialized);
        console.log('presaleAccountData.recipient_wallet', presaleAccountData.recipient_wallet);
        return;
      }
    }

    // Create the transaction to initialize the presale
    const tx = await program.methods.initialize(presaleToken, presaleSymbol, bump, new anchor.BN(startTime), new anchor.BN(endTime))
      .accounts({
        presaleAccount: presaleAccountPublicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers(walletKeypair)
      .rpc();
    
    console.log("Your transaction signature", tx);

    // Fetch the newly created presale account from the chain
    const presaleAccountData = await program.account.presaleAccount.fetch(presaleAccountPublicKey);

    // console.log('presaleAccountData.isActive', presaleAccountData);
    // console.log('presaleAccountData.isActive', presaleAccountData.isActive);
    // Add assertions to verify the presale account's data
    assert.ok(presaleAccountData.startTime.eq(new anchor.BN(startTime)));
    assert.ok(presaleAccountData.endTime.eq(new anchor.BN(endTime)));
    assert.ok(presaleAccountData.isActive === true);
  });
});
