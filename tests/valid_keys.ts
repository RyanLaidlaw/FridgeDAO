import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup/setup";
import { assert } from "chai";
import { expectAnchorError, createATA } from "./helpers/helpers";
import { getAssociatedTokenAddress, createAssociatedTokenAccount } from "@solana/spl-token";

describe("Valid Keys", async () => {
  let valid_keys: Array<anchor.web3.PublicKey>;
  let ctx;
  let txn;

  beforeEach(async () => {
    ctx = await setup();
    valid_keys = Array.from(
      { length: 5 },
      () => anchor.web3.Keypair.generate().publicKey
    );
  });

  it("Allows addition of valid keys", async () => {
    const { program, authority, fridgeDaoPda } = ctx;
    txn = await program.methods
      .addValidKeys(valid_keys)
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    for (let key of valid_keys) {
      assert(
        dao.validMemberKeys.some((a) => a.key.toBase58() == key.toBase58()),
        "Key not found in the dao"
      );
    }
  });

  it("Prevents adding existing keys", async () => {
    const { program, authority, fridgeDaoPda } = ctx;

    await program.methods
      .addValidKeys([valid_keys[0]])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    await expectAnchorError(
      program.methods
        .addValidKeys([valid_keys[0]])
        .accounts({
          adder: authority.publicKey,
          fridgeDao: fridgeDaoPda,
        })
        .rpc(),
        "KeyAlreadyAdded"
    );
  });

  it("Prevents non-authority people from adding", async () => {
    const { program, fridgeDaoPda } = ctx;

    const fake = anchor.web3.Keypair.generate();

    await expectAnchorError(
      program.methods
        .addValidKeys([anchor.web3.Keypair.generate().publicKey])
        .accounts({
          adder: fake.publicKey,
          fridgeDao: fridgeDaoPda,
        })
        .signers([fake])
        .rpc(),
        "InvalidAuthority"
    );
  });

  it("Prevents non-authority people from removing", async () => {
    const { program, fridgeDaoPda, usdcMint, vaultAuthPda, vault } = ctx;

    const fake = anchor.web3.Keypair.generate();

    await expectAnchorError(
      program.methods
        .removeValidKeys([valid_keys[0]])
        .accounts({
          remover: fake.publicKey,
          fridgeDao: fridgeDaoPda,
          vaultAuthority: vaultAuthPda,
          vault: vault,
          usdcMint: usdcMint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([fake])
        .rpc(),
        "InvalidAuthority"
    );
  });

  it("Allows for removal of keys", async () => {
    const { provider, program, authority, fridgeDaoPda, usdcMint, vaultAuthPda, vault } = ctx;

    let toRemove = anchor.web3.Keypair.generate().publicKey;
    txn = await program.methods
      .addValidKeys([toRemove])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();
    
    const userAta = await createATA(toRemove, provider, usdcMint, authority);

    txn = await program.methods
      .removeValidKeys([toRemove])
      .accounts({
        remover: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        vaultAuthority: vaultAuthPda,
        vault: vault,
        usdcMint: usdcMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .remainingAccounts([
        {
          pubkey: userAta,
          isWritable: true,
          isSigner: false,
        },
      ])
      .rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    assert(
      !dao.validMemberKeys.find(
        (a) => a.key.toBase58() === toRemove.toBase58()
      ),
      "Key still found in the dao"
    );
  });
});
