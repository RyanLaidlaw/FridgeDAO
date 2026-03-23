import * as anchor from "@coral-xyz/anchor";
import { FridgeDao } from "../../target/types/fridge_dao";
import { assert } from "chai";

export async function fund(acct: anchor.web3.PublicKey, provider: anchor.AnchorProvider) {
    const sig = await provider.connection.requestAirdrop(acct, 2 * anchor.web3.LAMPORTS_PER_SOL);

    await provider.connection.confirmTransaction(sig);
}

export async function createProposal(dao, program: anchor.Program<FridgeDao>, proposer, fridgeDaoPda) {
    const proposalId = dao.proposalCount.toNumber() + 1;
    
    const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
        Buffer.from("fridge_prop"),
        fridgeDaoPda.toBuffer(),
        new anchor.BN(proposalId).toArrayLike(Buffer, "le", 8),
        ],
        program.programId
    );
    
    let txn = await program.methods
        .propose("Soda")
        .accounts({
        proposer: proposer.publicKey,
        fridgeDao: fridgeDaoPda,
        proposal: proposalPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([proposer])
        .rpc();
        
    let proposal =  await program.account.proposal.fetch(proposalPda);

    return [proposal, proposalPda, proposalId];
}

export async function expectAnchorError(promise: Promise<any>, code: string) {
  try {
    await promise;
    assert.fail("Expected error but transaction succeeded");
  } catch (err: any) {
    assert.equal(err.error.errorCode.code, code);
  }
}