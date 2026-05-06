import { setup } from "../setup";
import { expectAnchorError, createATA, getProposalPda } from "./helpers/helpers";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { mintTo } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe.only("FridgeDAO - Cancel Proposals", async () => {
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

    let proposalPda: PublicKey;

    beforeEach(async () => {
        let {fridgeDaoProgram, fridgeDaoPda} = ctx;
        
        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;

        const [pda] = getProposalPda(fridgeDaoPda, fridgeDaoProgram, daoAccountBefore);
        proposalPda = pda

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

    it("Blocks invalid callers", async () => {
        const { fridgeDaoProgram, fridgeDaoPda, provider } = ctx;

        const invalidAuth = Keypair.generate();
        const sig = await provider.connection.requestAirdrop(invalidAuth.publicKey, 1e9);
        await provider.connection.confirmTransaction(sig);

        await expectAnchorError(
            fridgeDaoProgram.methods
                .cancelProposal()
                .accounts({
                    canceller: invalidAuth.publicKey,
                    fridgeDao: fridgeDaoPda,
                    proposal: proposalPda,
                    proposer: addedKey.publicKey,
                })
                .signers([invalidAuth])
                .rpc(),
            "InvalidAuthority"
        );
    });

    it("Allows admin to cancel", async () => {
        const { fridgeDaoProgram, fridgeDaoPda, authority, provider } = ctx;
        
        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;

        const balanceBefore = await provider.connection.getBalance(addedKey.publicKey);

        await fridgeDaoProgram.methods
            .cancelProposal()
            .accounts({
                canceller: authority.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
                proposer: addedKey.publicKey,
            })
            .rpc()
        
        const daoAccountAfter = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        
        const balanceAfter = await provider.connection.getBalance(addedKey.publicKey);
        assert(balanceAfter > balanceBefore, "Rent was not returned to the proposer")
    
        assert(daoAccountAfter.proposalCount.toNumber() === proposalCountBefore - 1, "Proposal count did not decrease by 1");
        assert(daoAccountAfter.proposals.length === proposalsLengthBefore - 1, "Proposals length did not decrease by 1");
    });

    it("Allows proposer to cancel", async () => {
        const { fridgeDaoProgram, fridgeDaoPda, provider } = ctx;
        
        const daoAccountBefore = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);
        let proposalCountBefore = daoAccountBefore.proposalCount.toNumber();
        let proposalsLengthBefore = daoAccountBefore.proposals.length;
        
        const balanceBefore = await provider.connection.getBalance(addedKey.publicKey);

        await fridgeDaoProgram.methods
            .cancelProposal()
            .accounts({
                canceller: addedKey.publicKey,
                fridgeDao: fridgeDaoPda,
                proposal: proposalPda,
                proposer: addedKey.publicKey,
            })
            .signers([addedKey])
            .rpc()
        
        const daoAccountAfter = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);

        const balanceAfter = await provider.connection.getBalance(addedKey.publicKey);
        assert(balanceAfter > balanceBefore, "Rent was not returned to the proposer")

        assert(daoAccountAfter.proposalCount.toNumber() === proposalCountBefore - 1, "Proposal count did not decrease by 1");
        assert(daoAccountAfter.proposals.length === proposalsLengthBefore - 1, "Proposals length did not decrease by 1");
    });
});