import { setup } from "../setup";
import { expectAnchorError, createATA, getProposalPda } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { mintTo } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("FridgeDAO - Proposing", async () => {
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

    it("Blocks invalid members", async () => {
        const { fridgeDaoProgram, fridgeDaoPda, provider } = ctx;

        const invalidAuth = Keypair.generate();
        const sig = await provider.connection.requestAirdrop(invalidAuth.publicKey, 1e9);
        await provider.connection.confirmTransaction(sig);

        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        const [proposalPda] = getProposalPda(fridgeDaoPda, fridgeDaoProgram, daoAccountBefore);

        await expectAnchorError(
            fridgeDaoProgram.methods
                .propose("Cookies", new anchor.BN(10))
                .accounts({
                    proposer: invalidAuth.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                    systemProgram: SYSTEM_PROGRAM_ID,
                })
                .signers([invalidAuth])
                .rpc(),
            "InvalidMember"
        );
    });

    it.skip("Blocks after max proposals", async () => {

    });

    it("Allows valid proposals", async () => {
        const { fridgeDaoProgram, fridgeDaoPda } = ctx;
        
        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;
        
        const [proposalPda] = getProposalPda(fridgeDaoPda, fridgeDaoProgram, daoAccountBefore);

        await fridgeDaoProgram.methods
            .propose("Cookies", new anchor.BN(10))
            .accounts({
                proposer: addedKey.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
                systemProgram: SYSTEM_PROGRAM_ID,
            })
            .signers([addedKey])
            .rpc()
        
        const daoAccountAfter = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);

        assert(daoAccountAfter.proposalCount.toNumber() === proposalCountBefore + 1, "Proposal count did not increase by 1");
        assert(daoAccountAfter.proposals.length === proposalsLengthBefore + 1, "Proposals length did not increase by 1");
    });
});