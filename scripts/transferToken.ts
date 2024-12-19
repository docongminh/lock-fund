import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { getProgram, getEscrowAccount, getConfigAccount } from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { LockFund } from "../target/types/lock_fund";

type TransferTokenParams = {
  mintToken: anchor.web3.PublicKey;
  amount: anchor.BN;
};
const transferToken = async (
  program: anchor.Program<LockFund>,
  authority: anchor.web3.Keypair,
  approver: anchor.web3.Keypair,
  params: TransferTokenParams
): Promise<string> => {
  const { mintToken, amount } = params;
  const escrow = getEscrowAccount(program);
  console.log(escrow);
  const configAccount = getConfigAccount(program);
  const configAccountData = await program.account.configAccount.fetch(
    configAccount
  );
  const recipientToken = getAssociatedTokenAddressSync(
    mintToken,
    configAccountData.recipient
  );

  const escrowToken = getAssociatedTokenAddressSync(mintToken, escrow, true);

  console.log({
    escrow: escrow.toString(),
    escrowToken: escrowToken.toString(),
    recipientToken: escrowToken.toString(),
  });

  const accounts = {
    configAccount,
    escrow,
    escrowToken,
    recipientToken,
    recipient: configAccountData.recipient,
    mintToken,
    authority: authority.publicKey,
    approver: approver.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  };
  return await program.methods
    .transferToken(amount)
    .accounts(accounts)
    .preInstructions([
      createAssociatedTokenAccountInstruction(
        authority.publicKey,
        recipientToken,
        configAccountData.recipient,
        mintToken,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      ),
    ])
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
  const amount = new anchor.BN(1000 * 10e6);

  const program = getProgram(connection, new anchor.Wallet(authority));

  const createConfigParams: TransferTokenParams = {
    mintToken: new anchor.web3.PublicKey(
      "9gTkRES3n4Tc3AZnRbTq9B3HWRuyshDXpfo7TDgigBsH"
    ),
    amount,
  };

  const signature = await transferToken(
    program,
    authority,
    approver,
    createConfigParams
  );
  console.log(`transaction signature: ${signature}`);
})();
