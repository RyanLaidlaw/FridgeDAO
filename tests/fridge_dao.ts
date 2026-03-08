import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { FridgeDao } from "../target/types/fridge_dao";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

describe("fridge_dao", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.fridgeDao as Program<FridgeDao>;
  const authority = provider.wallet;

  let fridgeDaoPda: anchor.web3.PublicKey;
  let fridgeDaoBump: number;

  let vaultAuthPda: anchor.web3.PublicKey;
  let vaultAuthBump: number;

  let usdcMint: anchor.web3.PublicKey;
  let vaultTokenAccount: anchor.web3.Keypair;

  let votingPeriod = 1000;
  let timeUntilStart = 100;

  let txn: string;
  let blockTime: number | null;

  let valid_addrs = Array.from( { length: 5 }, () => anchor.web3.Keypair.generate().publicKey);

  before(async () => {

    [fridgeDaoPda, fridgeDaoBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fridge")],
        program.programId
      );

    [vaultAuthPda, vaultAuthBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault_authority"), fridgeDaoPda.toBuffer()],
        program.programId
      );

    usdcMint = await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      null,
      6
    );

    vaultTokenAccount = anchor.web3.Keypair.generate();

    const slot = await provider.connection.getSlot();
    blockTime = await provider.connection.getBlockTime(slot);

    txn = await program.methods
      .initialize(new anchor.BN(votingPeriod), new anchor.BN(timeUntilStart))
      .accounts({
        authority: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        vaultAuthority: vaultAuthPda,
        vault: vaultTokenAccount.publicKey,
        usdcMint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vaultTokenAccount])
      .rpc();
  });

  it("Is correctly initialized", async () => {
    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);
    console.log("FridgeDao:", dao);

    assert.equal(dao.authority.toBase58(), authority.publicKey.toBase58(), "Authorities not equal");

    assert.equal(dao.vault.toBase58(), vaultTokenAccount.publicKey.toBase58(), "Treasuries not equal");

    assert.equal(dao.usdcMint.toBase58(), usdcMint.toBase58(), "USDC Mint accounts not equal");

    assert(dao.proposalCount.eq(new anchor.BN(0)), "Proposal Count is not cleared to 0");

    assert.equal(dao.bump, fridgeDaoBump, "Fridge Bumps not equal");
    assert.equal(dao.vaultBump, vaultAuthBump, "Vault Bumps not equal");

    assert(dao.votePeriod.eq(new anchor.BN(votingPeriod)), "Vote periods not equal");

    assert(new anchor.BN(blockTime!).lte(dao.nextVoteAllowedAt), "We are not past the voting start time");
  });

  it("Allows addition of valid addresses", async () => {
    txn = await program.methods.addValidAddresses(valid_addrs)
    .accounts({
      adder: authority.publicKey,
      fridgeDao: fridgeDaoPda,
    }).rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    for (let addr of valid_addrs) {
      assert(dao.validMemberAddresses.some(a => a.toBase58() == addr.toBase58()), "Address not found in the dao");
    }
  });

  it("Prevents adding duplicate addresses", async () => {
    try {
      txn = await program.methods.addValidAddresses([valid_addrs[0]])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      }).rpc();
      
      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  })

    it("Prevents non-authority people from adding", async () => {
    try {
      txn = await program.methods.addValidAddresses([anchor.web3.Keypair.generate().publicKey])
      .accounts({
        adder: anchor.web3.Keypair.generate().publicKey,
        fridgeDao: fridgeDaoPda,
      }).rpc();
      
      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  })

      it("Prevents non-authority people from removing", async () => {
    try {
      txn = await program.methods.removeValidAddresses([valid_addrs[0]])
      .accounts({
        adder: anchor.web3.Keypair.generate().publicKey,
        fridgeDao: fridgeDaoPda,
      }).rpc();
      
      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  })

  it("Allows for removal of addresses", async () => {
    let toRemove = anchor.web3.Keypair.generate().publicKey;
    txn = await program.methods.addValidAddresses([toRemove])
    .accounts({
      adder: authority.publicKey,
      fridgeDao: fridgeDaoPda,
    }).rpc();

    txn = await program.methods.removeValidAddresses([toRemove])
    .accounts({
      remover: authority.publicKey,
      fridgeDao: fridgeDaoPda,
    }).rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    assert(!dao.validMemberAddresses.find(a => a.toBase58() === toRemove.toBase58()), "Address still found in the dao");
  });
});