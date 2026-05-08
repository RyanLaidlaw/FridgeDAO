import { setup } from "../setup";
import { expectAnchorError, createATA, getProposalPda } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { mintTo } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("FridgeDAO - Voting", async () => {
    let ctx;
    let user1Key: Keypair;
    let user1Ata: PublicKey;
    let user2Key: Keypair;
    let user2Ata: PublicKey;
    let proposalPda: PublicKey;
    let proposalCount: number;

    before(async () => {
        ctx = await setup();

        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, usdcMint } = ctx;

        user1Key = Keypair.generate();
        user1Ata = await createATA(user1Key.publicKey, provider, usdcMint, authority)

        user2Key = Keypair.generate();
        user2Ata = await createATA(user2Key.publicKey, provider, usdcMint, authority)

        let txn = await daoMintProgram.methods
            .whitelist([user1Key.publicKey, user2Key.publicKey])
            .accounts({
                adder: authority.publicKey,
                daoMint: daoMintPda,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda
            })
            .remainingAccounts(
                [{
                    pubkey: user1Ata, isWritable: true, isSigner: false,
                }, {
                    pubkey: user2Ata, isWritable: true, isSigner: false,
                }
            ])
            .rpc();
        
        await provider.connection.confirmTransaction(txn);

        await mintTo(
            provider.connection,
            authority.payer,
            usdcMint,
            user1Ata,
            authority.payer,
            1_000_000_000
        );

        const sig = await provider.connection.requestAirdrop(user1Key.publicKey, 1e9);
        await provider.connection.confirmTransaction(sig);

        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;

        const [pda] = getProposalPda(fridgeDaoPda, fridgeDaoProgram, daoAccountBefore);
        proposalPda = pda;

        await fridgeDaoProgram.methods
            .propose("Cookies", new anchor.BN(10))
            .accounts({
                proposer: user1Key.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
                systemProgram: SYSTEM_PROGRAM_ID,
            })
            .signers([user1Key])
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

    it("Blocks voting for same thing more than once", async () => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;

        await fridgeDaoProgram.methods
            .vote(new anchor.BN(proposalCount))
            .accounts({
                voter: user1Key.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
            })
            .signers([user1Key])
            .rpc();
        
        await expectAnchorError(
            fridgeDaoProgram.methods
                .vote(new anchor.BN(1))
                .accounts({
                    voter: user1Key.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([user1Key])
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
                    voter: user1Key.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                })
                .signers([user1Key])
                .rpc(),
            "CouldNotFindProposal"
        );
    });

    it("Allows valid votes", async () => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;

        const proposalBefore = await fridgeDaoProgram.account.proposal.fetch(proposalPda);
        const votersBefore = proposalBefore.voters.length;

        await fridgeDaoProgram.methods
            .vote(new anchor.BN(proposalCount))
            .accounts({
                voter: user2Key.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
            })
            .signers([user2Key])
            .rpc();

        const proposalAfter = await fridgeDaoProgram.account.proposal.fetch(proposalPda);
        assert(proposalAfter.voters.length === votersBefore + 1, "Voter was not added");
        assert(proposalAfter.voters.some(v => v.equals(user2Key.publicKey)), "user2 not found in voters");
    });
});