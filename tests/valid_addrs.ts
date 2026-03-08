import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup/setup";
import { assert } from "chai";

describe("Valid Addresses", async () => {
  let valid_addrs: Array<anchor.web3.PublicKey>;
  let ctx;
  let txn;

  beforeEach(async () => {
    ctx = await setup();
    valid_addrs = Array.from(
      { length: 5 },
      () => anchor.web3.Keypair.generate().publicKey
    );
  });

  it("Allows addition of valid addresses", async () => {
    const { program, authority, fridgeDaoPda } = ctx;
    txn = await program.methods
      .addValidAddresses(valid_addrs)
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    for (let addr of valid_addrs) {
      assert(
        dao.validMemberAddresses.some((a) => a.toBase58() == addr.toBase58()),
        "Address not found in the dao"
      );
    }
  });

  it("Prevents adding duplicate addresses", async () => {
    const { program, authority, fridgeDaoPda } = ctx;

    try {
      txn = await program.methods
        .addValidAddresses([valid_addrs[0]])
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
        .addValidAddresses([anchor.web3.Keypair.generate().publicKey])
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
        .removeValidAddresses([valid_addrs[0]])
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

  it("Allows for removal of addresses", async () => {
    const { program, authority, fridgeDaoPda } = ctx;

    let toRemove = anchor.web3.Keypair.generate().publicKey;
    txn = await program.methods
      .addValidAddresses([toRemove])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    txn = await program.methods
      .removeValidAddresses([toRemove])
      .accounts({
        remover: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    assert(
      !dao.validMemberAddresses.find(
        (a) => a.toBase58() === toRemove.toBase58()
      ),
      "Address still found in the dao"
    );
  });
});
