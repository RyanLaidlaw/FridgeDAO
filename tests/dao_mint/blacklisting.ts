import { setup } from "../setup";
import { expectAnchorError, createATA } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";

describe("DAO Mint - Blacklisting", async () => {
    let ctx;
    let addedKey: Keypair;
    let addedAta: any;

    before(async () => {
        ctx = await setup();

        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, usdcMint } = ctx;

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
    });

    it("blocks invalid authority", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, fridgeDaoPda, vault, usdcMint } = ctx;

        const invalidAuth = Keypair.generate();

        await expectAnchorError(
            daoMintProgram.methods
                .blacklist([addedKey.publicKey])
                .accounts({
                    remover: invalidAuth.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda,
                    vault: vault,
                    usdcMint: usdcMint,
                    tokenProgram: tokenProgram
                })
                .signers([invalidAuth])
                .remainingAccounts(
                    [{
                        pubkey: addedAta, isWritable: true, isSigner: false 
                    }]
                )
                .rpc(),
            "InvalidAuthority"
        );
    });

    it("blocks when no ATAs are supplied", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        await expectAnchorError(
            daoMintProgram.methods
                .blacklist([addedKey.publicKey])
                .accounts({
                    remover: authority.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda,
                    vault: vault,
                    usdcMint: usdcMint,
                    tokenProgram: tokenProgram
                })
                .rpc(),
            "MismatchingRemainingAccountsLength"
        );
    });

    it("blocks for duplicate keys", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        await expectAnchorError(
            daoMintProgram.methods
                .blacklist([addedKey.publicKey, addedKey.publicKey])
                .accounts({
                    remover: authority.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda,
                    vault: vault,
                    usdcMint: usdcMint,
                    tokenProgram: tokenProgram
                })
                .remainingAccounts(
                    [{
                        pubkey: addedAta, isWritable: true, isSigner: false 
                    },
                    {
                        pubkey: addedAta, isWritable: true, isSigner: false 
                    }]
                )
                .rpc(),
            "DuplicateKeys"
        );
    });

    it("blocks for blacklisting non-existent keys", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        const nonExistentKey = Keypair.generate();
        const userAta = await createATA(nonExistentKey.publicKey, provider, usdcMint, authority)

        await expectAnchorError(
            daoMintProgram.methods
                .blacklist([nonExistentKey.publicKey])
                .accounts({
                    remover: authority.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda,
                    vault: vault,
                    usdcMint: usdcMint,
                    tokenProgram: tokenProgram
                })
                .remainingAccounts([{
                    pubkey: userAta, isWritable: true, isSigner: false
                }])
                .rpc(),
            "KeyDoesNotExist"
        );
    });

    it("blocks for invalid token account owners", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        const anotherKey = Keypair.generate();
        const anotherAta = await createATA(anotherKey.publicKey, provider, usdcMint, authority)

        let txn = await daoMintProgram.methods
            .whitelist([anotherKey.publicKey])
            .accounts({
                adder: authority.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda
            })
            .remainingAccounts(
                [{
                    pubkey: anotherAta, isWritable: true, isSigner: false 
                }])
            .rpc();
        
        await provider.connection.confirmTransaction(txn);

        await expectAnchorError(
            daoMintProgram.methods
                .blacklist([anotherKey.publicKey])
                .accounts({
                    remover: authority.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda,
                    vault: vault,
                    usdcMint: usdcMint,
                    tokenProgram: tokenProgram
                })
                .remainingAccounts([{
                    pubkey: addedAta, isWritable: true, isSigner: false
                }])
                .rpc(),
            "InvalidTokenAccountOwner"
        );
    });

    it("correctly returns funds on blacklisting", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;
    });

    it("correctly blacklists keys", async () => {
        const { daoMintProgram, fridgeDaoProgram, tokenProgram, daoMintPda, fridgeDaoPda, vault, authority, usdcMint } = ctx;

        await daoMintProgram.methods
            .blacklist([addedKey.publicKey])
            .accounts({
                remover: authority.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda,
                vault: vault,
                usdcMint: usdcMint,
                tokenProgram: tokenProgram
            })
            .remainingAccounts([{
                pubkey: addedAta, isWritable: true, isSigner: false
            }])
            .rpc()
        
        const dao_mint = await daoMintProgram.account.daoMint.fetch(daoMintPda);
        const fridgeDao = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);

        assert(
            !dao_mint.validMemberKeys.some((u: any) => 
                u.key.toBase58() === addedKey.publicKey.toBase58()),
            "User not removed from DAO mint"
        );

        assert(
            !fridgeDao.validMemberKeys.some((u: any) => 
                u.key.toBase58() === addedKey.publicKey.toBase58()),
            "User not removed from fridge DAO"
        );
    });
});