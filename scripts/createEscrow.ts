import * as anchor from "@coral-xyz/anchor";
import { getEscrowPda, getEscrowProgram, getEscrowVaultPda } from "./setup";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

(async () => {
  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet")
  );
  // params for create escrow
  const approver = new anchor.web3.PublicKey(
    "6Fdm2kkx3utt2SLMGaGDChRkfH2awve4fxkPnAZbwgc2"
  );
  const recipient = new anchor.web3.PublicKey(
    "CPb3FSzbsHv5tkKuWzgT7xKMxa2DwnBRnFMHY46jaopy"
  );
  const cliffTime = new anchor.BN(0); //
  const amountPerDay = new anchor.BN(1000 * 10e6);
  const updateActorMode = 1;
  const enableWithdrawlFull = 0;
  ///
  const authority = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(require("./keys/authority.json"))
  );
  const program = getEscrowProgram(connection, new anchor.Wallet(authority));
  const lockFundEscrow = getEscrowPda(program);
  const escrowVault = getEscrowVaultPda(program);

  const accounts = {
    authority: authority.publicKey,
    lockFundEscrow,
    escrowVault,
    recipient,
    approver,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  };
  const signature = await program.methods
    .createLockFundEscrow({
      cliffTime,
      amountPerDay,
      updateActorMode,
      enableWithdrawlFull,
    })
    .accounts(accounts)
    .signers([authority])
    .rpc();

  console.log(`transaction signature: ${signature}`);
})();
