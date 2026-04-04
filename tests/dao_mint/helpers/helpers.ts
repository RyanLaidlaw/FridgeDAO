import { assert } from "chai";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

export async function expectAnchorError(promise: Promise<any>, code: string) {
  try {
    await promise;
    assert.fail("Expected error but transaction succeeded");
  } catch (err: any) {
    assert.equal(err.error.errorCode.code, code);
  }
}

export async function createATA(user, provider, usdcMint, authority) {

  const userAta = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    authority.payer,
    usdcMint,
    user
  );

  return userAta.address;
}