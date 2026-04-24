import { setup } from "./setup/setup";
import { expectAnchorError, createATA } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { mintTo, getAccount } from "@solana/spl-token";

describe("Withdrawing", async () => {
    let ctx;
    let addedKey: Keypair;
    let addedAta: any;

    before(async () => {
        ctx = await setup();

        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, vault, usdcMint } = ctx;

        const newKey = Keypair.generate();
        addedKey = newKey;
        const userAta = await createATA(newKey.publicKey, provider, usdcMint, authority)
        addedAta = userAta;

        let txn = await daoMintProgram.methods
            .whitelist([newKey.publicKey])
            .accounts({
                adder: authority.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda
            })
            .remainingAccounts(
                [{
                    pubkey: userAta, isWritable: true, isSigner: false 
                }])
            .rpc();
        
        await provider.connection.confirmTransaction(txn);

        await mintTo(
            provider.connection,
            authority.payer,
            usdcMint,
            addedAta,
            authority.payer,
            1_000_000_000
        );

        const sig = await provider.connection.requestAirdrop(addedKey.publicKey, 1e9);
        await provider.connection.confirmTransaction(sig);
    });

    it("blocks invalid user calling someone elses ata", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, fridgeDaoPda, vault, usdcMint } = ctx;

        const invalidAuth = Keypair.generate();

        await expectAnchorError(
            daoMintProgram.methods
            .withdraw(new anchor.BN(100000000))
            .accounts({
                withdrawer: invalidAuth.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda,
                vault: vault,
                usdcMint: usdcMint,
                userTokenAccount: addedAta,
                tokenProgram: tokenProgram
            })
            .signers([invalidAuth])
            .rpc(),
            "InvalidTokenAccount"
        );
    });

    it("blocks invalid users", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, provider, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        const invalidAuth = Keypair.generate();
        const invalidAta = await createATA(invalidAuth.publicKey, provider, usdcMint, authority)

        await expectAnchorError(
            daoMintProgram.methods
            .withdraw(new anchor.BN(100000000))
            .accounts({
                withdrawer: invalidAuth.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda,
                vault: vault,
                usdcMint: usdcMint,
                userTokenAccount: invalidAta,
                tokenProgram: tokenProgram
            })
            .signers([invalidAuth])
            .rpc(),
            "InvalidMember"
        );
    });

    it("allows withdrawals", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, provider, fridgeDaoPda, vault, usdcMint } = ctx;
        
        const vaultAccountBefore = await getAccount(provider.connection, vault);
        const vaultBalanceBefore = vaultAccountBefore.amount;

        await daoMintProgram.methods
        .deposit(new anchor.BN(100000000))
        .accounts({
            depositor: addedKey.publicKey,
            daoMint: daoMintPda,
            daoProgram: fridgeDaoProgram.programId,
            fridgeDao: fridgeDaoPda,
            vault: vault,
            usdcMint: usdcMint,
            userTokenAccount: addedAta,
            tokenProgram: tokenProgram
        })
        .signers([addedKey])
        .rpc();

        const ataAccount = await getAccount(provider.connection, addedAta);
        assert(
            ataAccount.amount === BigInt(1_000_000_000 - 100_000_000),
            "ATA balance not reduced correctly"
        );

        const vaultAccount = await getAccount(provider.connection, vault);
        assert(
            vaultAccount.amount === vaultBalanceBefore + BigInt(100_000_000),
            "Vault balance not incremented correctly"
        );

        const fridgeDao = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
                assert(
            fridgeDao.validMemberKeys.some(u => {
                    if (u.key.toBase58() === addedKey.publicKey.toBase58()) {
                        return u.balance.eq(new anchor.BN(100000000));
                    }
                    return false;
                }
            ),
            "User funds not added to DAO"
        );

        await daoMintProgram.methods
        .withdraw(new anchor.BN(100000000))
        .accounts({
            withdrawer: addedKey.publicKey,
            daoMint: daoMintPda,
            daoProgram: fridgeDaoProgram.programId,
            fridgeDao: fridgeDaoPda,
            vault: vault,
            usdcMint: usdcMint,
            userTokenAccount: addedAta,
            tokenProgram: tokenProgram
        })
        .signers([addedKey])
        .rpc();

        const newAtaState = await getAccount(provider.connection, addedAta);
        assert(
            newAtaState.amount === BigInt(1_000_000_000),
            "ATA balance not increased correctly"
        );

        const newVaultState = await getAccount(provider.connection, vault);
        assert(
            newVaultState.amount === vaultBalanceBefore,
            "Vault balance not decremented correctly"
        );

        const newDaoState = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        assert(
            newDaoState.validMemberKeys.some(u => {
                    if (u.key.toBase58() === addedKey.publicKey.toBase58()) {
                        return u.balance.eq(new anchor.BN(0));
                    }
                    return false;
                }
            ),
            "User funds not removed from DAO"
        );

    });
});