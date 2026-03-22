import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup/setup";
import { assert } from "chai";

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

  it("Prevents adding duplicate keys", async () => {
    const { program, authority, fridgeDaoPda } = ctx;

    try {
      txn = await program.methods
        .addValidKeys([valid_keys[0]])
        .accounts({
          adder: authority.publicKey,
          fridgeDao: fridgeDaoPda,
        })
        .rpc();

      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  });

  it("Prevents non-authority people from adding", async () => {
    const { program, fridgeDaoPda } = ctx;
    try {
      txn = await program.methods
        .addValidKeys([anchor.web3.Keypair.generate().publicKey])
        .accounts({
          adder: anchor.web3.Keypair.generate().publicKey,
          fridgeDao: fridgeDaoPda,
        })
        .rpc();

      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  });

  it("Prevents non-authority people from removing", async () => {
    const { program, fridgeDaoPda } = ctx;

    try {
      txn = await program.methods
        .removeValidKeys([valid_keys[0]])
        .accounts({
          adder: anchor.web3.Keypair.generate().publicKey,
          fridgeDao: fridgeDaoPda,
        })
        .rpc();

      assert.fail("Txn did not revert");
    } catch (err) {
      assert.ok(err);
    }
  });

  it("Allows for removal of keys", async () => {
    const { program, authority, fridgeDaoPda } = ctx;

    let toRemove = anchor.web3.Keypair.generate().publicKey;
    txn = await program.methods
      .addValidKeys([toRemove])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    txn = await program.methods
      .removeValidKeys([toRemove])
      .accounts({
        remover: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
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
