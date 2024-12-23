import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { getProgram, getEscrowAccount, getConfigAccount } from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { LockFund } from "../target/types/lock_fund";

const transferSOL = async (
  program: anchor.Program<LockFund>,
  authority: anchor.web3.Keypair,
  approver: anchor.web3.Keypair,
  amount: anchor.BN
): Promise<string> => {
  const escrow = getEscrowAccount(program);
  const configAccount = getConfigAccount(program);
  const configAccountData = await program.account.configAccount.fetch(
    configAccount
  );

  const accounts = {
    configAccount,
    escrow,
    recipient: configAccountData.recipient,
    authority: authority.publicKey,
    approver: approver.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  };
  return await program.methods
    .transferSol(amount)
    .accounts(accounts)
    .signers([authority, approver])
    .rpc();
};

(async () => {
  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet")
  );
  /// Setup Accounts
  const authority = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(require("./keys/authority.json"))
  );
  const approver = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(require("./keys/approver.json"))
  );

  // Define Params
  const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);

  const program = getProgram(connection, new anchor.Wallet(authority));

  const signature = await transferSOL(program, authority, approver, amount);
  console.log(`transaction signature: ${signature}`);
})();
