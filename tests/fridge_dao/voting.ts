import { setup } from "../setup";
import { expectAnchorError, createATA, getProposalPda } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { mintTo } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("FridgeDAO - Voting", async () => {
    let ctx;
    let addedKey: Keypair;
    let addedAta: any;
    let proposalPda: PublicKey;
    let proposalCount: Number;

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

        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;

        const [pda] = getProposalPda(fridgeDaoPda, fridgeDaoProgram, daoAccountBefore);
        proposalPda = pda;

        await fridgeDaoProgram.methods
            .propose("Cookies", new anchor.BN(10))
            .accounts({
                proposer: addedKey.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
                systemProgram: SYSTEM_PROGRAM_ID,
            })
            .signers([addedKey])
            .rpc();
        
        const daoAccountAfter = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        proposalCount = daoAccountAfter.proposalCount.toNumber();
        assert(proposalCount === proposalCountBefore + 1, "Proposal count did not increase by 1");
        assert(daoAccountAfter.proposals.length === proposalsLengthBefore + 1, "Proposals length did not increase by 1");
    });

    it("Blocks invalid members", async () => {
        let {fridgeDaoProgram, fridgeDaoPda, provider} = ctx;
        
        const invalidAuth = Keypair.generate();

        let sig = await provider.connection.requestAirdrop(invalidAuth.publicKey, 1e9);
        await provider.connection.confirmTransaction(sig);

        await expectAnchorError(
            fridgeDaoProgram.methods
                .vote(new anchor.BN(proposalCount))
                .accounts({
                    voter: invalidAuth.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([invalidAuth])
                .rpc(),
            "InvalidMember"
        );
    });

    it.skip("Blocks voting for same thing more than once", async () => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;

        await fridgeDaoProgram.methods
            .vote(new anchor.BN(proposalCount))
            .accounts({
                voter: addedKey.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
            })
            .signers([addedKey])
            .rpc();
        
        await expectAnchorError(
            fridgeDaoProgram.methods
                .vote(new anchor.BN(1))
                .accounts({
                    voter: addedKey.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([addedKey])
                .rpc(),
            "AlreadyVoted"
        );
    });

    it("Blocks voting for invalid proposal", async () => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;

        await expectAnchorError(
            fridgeDaoProgram.methods
                .vote(new anchor.BN(proposalCount + 1))
                .accounts({
                    voter: addedKey.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([addedKey])
                .rpc(),
            "CouldNotFindProposal"
        );
    });

    it("Blocks voting outside of vote period", async() => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;

        await expectAnchorError(
            fridgeDaoProgram.methods
                .vote(new anchor.BN(proposalCount))
                .accounts({
                    voter: addedKey.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([addedKey])
                .rpc(),
            "NotInVotingPeriod"
        );
    });

    it("Allows valid votes", async () => {

    });
});