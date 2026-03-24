import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup/setup";
import { assert } from "chai";
import { createATA } from "./helpers/helpers";
import { getAccount, mintTo } from "@solana/spl-token";

describe("Vault operations (Deposit/Withdraw)", async () => {
    let valid_keys: Array<anchor.web3.PublicKey>;
    let users: Array<anchor.web3.Keypair>;
    let ctx;
    let txn;

    before(async () => {
    ctx = await setup();
    users = Array.from(
        { length: 5 },
        () => anchor.web3.Keypair.generate()
    );

    valid_keys = users.map(u => u.publicKey);

    const { program, authority, fridgeDaoPda } = ctx;
    txn = await program.methods
    .addValidKeys(valid_keys)
    .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
    })
    .rpc();

    });

    it("allows for deposits", async () => {
        const { program, authority, fridgeDaoPda, usdcMint, provider, vault } = ctx;

        const user = users[0];

        const info = await provider.connection.getAccountInfo(user.publicKey);
        if (!info) {
        await provider.connection.requestAirdrop(user.publicKey, 1e9);
        }

        const userAta = await createATA(user.publicKey, provider, usdcMint, authority);

        await mintTo(
            provider.connection,
            authority.payer,
            usdcMint,
            userAta,
            authority.payer,
            1_000_000
        );

        const acctBefore = await getAccount(provider.connection, userAta);
        const balanceBefore = Number(acctBefore.amount);

        txn = await program.methods
        .deposit(new anchor.BN(1_000_000))
        .accounts({
            depositor: user.publicKey,
            fridgeDao: fridgeDaoPda,
            vault: vault,
            userTokenAccount: userAta,
            usdcMint: usdcMint,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();

        const acctAfter = await getAccount(provider.connection, userAta);
        const balanceAfter = Number(acctAfter.amount);

        const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

        assert.equal(dao.validMemberKeys[0].balance, balanceBefore - 1_000_000, "DAO user does not have the correct balance");
        assert.equal(balanceAfter, balanceBefore - 1_000_000, "User ATA does not have the correct balance");
    });
});