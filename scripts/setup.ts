import * as anchor from "@coral-xyz/anchor";
import { LockFund } from "../target/types/lock_fund";
import * as idl from "../target/idl/lock_fund.json";

const getSeed = (seed: string, program: anchor.Program<LockFund>): Buffer => {
  return Buffer.from(
    JSON.parse(program.idl.constants.find((c) => c.name === seed)!.value)
  );
};

export function getEscrowProgram(
  connection: anchor.web3.Connection,
  wallet?: anchor.Wallet
): anchor.Program<LockFund> {
  const provider = new anchor.AnchorProvider(connection, wallet, {
    maxRetries: 5,
    commitment: "confirmed",
  });
  return new anchor.Program<LockFund>(idl as LockFund, provider);
}

export function getEscrowPda(program: anchor.Program<LockFund>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [getSeed("escrowSeed", program), program.provider.publicKey.toBuffer()],
    program.programId
  )[0];
}

export function getEscrowVaultPda(program: anchor.Program<LockFund>) {
  const escrowPda = getEscrowPda(program);
  return anchor.web3.PublicKey.findProgramAddressSync(
    [getSeed("escrowVaultSeed", program), escrowPda.toBuffer()],
    program.programId
  )[0];
}
