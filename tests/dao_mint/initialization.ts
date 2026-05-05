import * as anchor from "@coral-xyz/anchor";
import { setup } from "../setup";
import { assert } from "chai";

describe("DAO Mint - Initialization", async () => {
    let ctx;
    before(async () => {ctx = await setup();});

    it("is initialized correctly", async () => {
        const { daoMintProgram, fridgeDaoProgram, provider, daoMintPda, fridgeDaoPda, authority, vault, usdcMint } = ctx;

        const dao_mint = await daoMintProgram.account.daoMint.fetch(daoMintPda);
        const fridgeDao = await fridgeDaoProgram.account.fridgeDao.fetch(fridgeDaoPda);

        assert.equal(
            dao_mint.admin.toBase58(),
            authority.publicKey.toBase58(),
            "Authorities not equal"
        )

        assert.equal(
            dao_mint.usdcMint.toBase58(),
            usdcMint.toBase58(),
            "USDC Mint accounts not equal"
        );

        assert(
            fridgeDao.proposalCount.eq(new anchor.BN(0)),
            "Proposal Count is not cleared to 0"
        );

        assert(
            fridgeDao.votePeriod.eq(new anchor.BN(1000)),
            "Vote periods not equal"
        );
        
        let slot = await provider.connection.getSlot();
        let blockTime = await provider.connection.getBlockTime(slot);

        assert(
            new anchor.BN(blockTime!).lte(fridgeDao.nextVoteAllowedAt),
            "We are not past the voting start time"
        );
    });
});