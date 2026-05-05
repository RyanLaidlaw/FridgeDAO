import { setup } from "../setup";
import { expectAnchorError, createATA } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";

describe("DAO Mint - Whitelisting", async () => {
    let ctx;
    before(async () => {ctx = await setup();});

    it("blocks invalid authority", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, usdcMint } = ctx;

        const invalidAuth = Keypair.generate();
        const newKey = Keypair.generate();

        const userAta = await createATA(newKey.publicKey, provider, usdcMint, authority)

        await expectAnchorError(
            daoMintProgram.methods
                .whitelist([newKey.publicKey])
                .accounts({
                    adder: invalidAuth.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda
                })
                .signers([invalidAuth])
                .remainingAccounts(
                    [{
                        pubkey: userAta, isWritable: true, isSigner: false 
                    }]
                )
                .rpc(),
            "InvalidAuthority"
        )
    });

    it("blocks when no ATAs are supplied", async () => {
        const { daoMintProgram, fridgeDaoProgram, daoMintPda, fridgeDaoPda } = ctx;

        const invalidAuth = Keypair.generate();
        const newKey = Keypair.generate();

        await expectAnchorError(
            daoMintProgram.methods
                .whitelist([newKey.publicKey])
                .accounts({
                    adder: invalidAuth.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda
                })
                .signers([invalidAuth])
                .rpc(),
            "MismatchingRemainingAccountsLength"
        )
    });

    it("blocks for duplicate keys", async () => {
        const { daoMintProgram, fridgeDaoProgram, daoMintPda, fridgeDaoPda, provider, usdcMint, authority } = ctx;

        const newKey = Keypair.generate();
        const userAta = await createATA(newKey.publicKey, provider, usdcMint, authority)
        const sameKey = newKey;
        const sameAta = await createATA(newKey.publicKey, provider, usdcMint, authority)

        await expectAnchorError(
            daoMintProgram.methods
                .whitelist([newKey.publicKey, sameKey.publicKey])
                .accounts({
                    adder: authority.publicKey,
                    daoMint: daoMintPda,
                    daoProgram: fridgeDaoProgram.programId,
                    fridgeDao: fridgeDaoPda
                })
                .remainingAccounts(
                    [{
                        pubkey: userAta, isWritable: true, isSigner: false 
                    }, {
                        pubkey: sameAta, isWritable: true, isSigner: false 
                }])
                .rpc(),
            "DuplicateKeys"
        )
    });

    it("blocks adding a key again", async () => {
        const { daoMintProgram, fridgeDaoProgram, daoMintPda, fridgeDaoPda, provider, usdcMint, authority } = ctx;

        const newKey = Keypair.generate();
        const userAta = await createATA(newKey.publicKey, provider, usdcMint, authority)

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
            .rpc()
        
        const dao_mint = await daoMintProgram.account.daoMint.fetch(daoMintPda);

        assert(
            dao_mint.validMemberKeys.some(u => 
                u.key.toBase58() === newKey.publicKey.toBase58() &&
                u.tokenAccount.toBase58() === userAta.toBase58()
            ),
            "User not added to DAO mint"
        );

        await expectAnchorError(
            daoMintProgram.methods
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
                    }
                ])
                .rpc(),
            "KeyAlreadyAdded"
        )
        
    });

    it("correctly adds to whitelist", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, vault, usdcMint } = ctx;

        const newKey = Keypair.generate();
        const userAta = await createATA(newKey.publicKey, provider, usdcMint, authority)

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
            .rpc()
        
        await provider.connection.confirmTransaction(txn);
        
        const dao_mint = await daoMintProgram.account.daoMint.fetch(daoMintPda);
        const fridgeDao = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);

        assert(
            dao_mint.validMemberKeys.some(u => 
                u.key.toBase58() === newKey.publicKey.toBase58() &&
                u.tokenAccount.toBase58() === userAta.toBase58()
            ),
            "User not added to DAO mint"
        );

        assert(
            fridgeDao.validMemberKeys.some(u =>
                u.key.toBase58() === newKey.publicKey.toBase58()
            ),
            "User not added to DAO"
        );
    });
});