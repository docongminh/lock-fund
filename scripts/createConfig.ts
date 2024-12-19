import * as anchor from "@coral-xyz/anchor";
import { getProgram, getEscrowAccount, getConfigAccount } from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { LockFund } from "../target/types/lock_fund";

type CreateConfigParams = {
  recipient: anchor.web3.PublicKey;
  approver: anchor.web3.PublicKey;
  cliffTimeDuration: anchor.BN;
  amountPerDay: anchor.BN;
  updateActorMode: number;
  enableTransferFull: number;
};
const createConfig = async (
  program: anchor.Program<LockFund>,
  authority: anchor.web3.Keypair,
  params: CreateConfigParams
): Promise<string> => {
  const {
    recipient,
    approver,
    cliffTimeDuration,
    amountPerDay,
    updateActorMode,
    enableTransferFull,
  } = params;
  const escrow = getEscrowAccount(program);
  const configAccount = getConfigAccount(program);

  const accounts = {
    authority: authority.publicKey,
    configAccount,
    escrow,
    recipient,
    approver,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  };
  return await program.methods
    .createConfig({
      cliffTimeDuration,
      amountPerDay,
      updateActorMode,
      enableTransferFull,
    })
    .accounts(accounts)
    .signers([authority])
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
  const approver = new anchor.web3.PublicKey(
    "6Fdm2kkx3utt2SLMGaGDChRkfH2awve4fxkPnAZbwgc2"
  );
  const recipient = new anchor.web3.PublicKey(
    "CPb3FSzbsHv5tkKuWzgT7xKMxa2DwnBRnFMHY46jaopy"
  );

  // Define Params
  const cliffTimeDuration = new anchor.BN(0); //
  const amountPerDay = new anchor.BN(1000 * 10e6);
  const updateActorMode = 1;
  const enableTransferFull = 0;

  const program = getProgram(connection, new anchor.Wallet(authority));

  const createConfigParams: CreateConfigParams = {
    recipient,
    approver,
    cliffTimeDuration,
    amountPerDay,
    updateActorMode,
    enableTransferFull,
  };

  const signature = await createConfig(program, authority, createConfigParams);
  console.log(`transaction signature: ${signature}`);
})();
