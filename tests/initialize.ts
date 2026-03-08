import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup/setup";
import { assert } from "chai";

describe("initializaton", async () => {
  let ctx;
  beforeEach(async () => {
    ctx = await setup();
  });

  it("Is correctly initialized", async () => {
    const {
      program,
      authority,
      usdcMint,
      fridgeDaoPda,
      fridgeDaoBump,
      vaultTokenAccount,
      vaultAuthBump,
      blockTime,
      votingPeriod,
    } = ctx;

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);
    console.log("FridgeDao:", dao);

    assert.equal(
      dao.authority.toBase58(),
      authority.publicKey.toBase58(),
      "Authorities not equal"
    );

    assert.equal(
      dao.vault.toBase58(),
      vaultTokenAccount.publicKey.toBase58(),
      "Treasuries not equal"
    );

    assert.equal(
      dao.usdcMint.toBase58(),
      usdcMint.toBase58(),
      "USDC Mint accounts not equal"
    );

    assert(
      dao.proposalCount.eq(new anchor.BN(0)),
      "Proposal Count is not cleared to 0"
    );

    assert.equal(dao.bump, fridgeDaoBump, "Fridge Bumps not equal");
    assert.equal(dao.vaultBump, vaultAuthBump, "Vault Bumps not equal");

    assert(
      dao.votePeriod.eq(new anchor.BN(votingPeriod)),
      "Vote periods not equal"
    );

    assert(
      new anchor.BN(blockTime!).lte(dao.nextVoteAllowedAt),
      "We are not past the voting start time"
    );
  });
});
