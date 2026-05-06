import { assert } from "chai";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";

export async function expectAnchorError(promise: Promise<any>, code: string) {
  try {
    await promise;
    assert.fail("Expected error but transaction succeeded");
  } catch (err: any) {
    if (!err.error) {
      console.log("Not an AnchorError:", err.message);
      throw err;
    }
    assert.equal(err.error.errorCode.code, code);
  }
}

export async function createATA(user: PublicKey, provider: anchor.AnchorProvider, usdcMint: PublicKey, authority) {

  const userAta = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    authority.payer,
    usdcMint,
    user
  );

  return userAta.address;
}

export function getProposalPda(fridgeDaoPda: PublicKey, fridgeDaoProgram, daoAccount) {
  return PublicKey.findProgramAddressSync(
      [
          Buffer.from("fridge_prop"),
          fridgeDaoPda.toBuffer(),
          new anchor.BN(daoAccount.proposalCount.toNumber() + 1).toArrayLike(Buffer, "le", 8),
      ],
      fridgeDaoProgram.programId
  );
}