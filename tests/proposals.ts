import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import { setup } from "./setup/setup";
import { fund } from "./helpers/helpers";

describe("proposals", () => {
  let proposer1: anchor.web3.Keypair;

  let txn: string;
  let ctx;

  before(async () => {
    ctx = await setup();
    proposer1 = anchor.web3.Keypair.generate();
  });

  it("Can create proposals", async () => {
    const { provider, program, authority, fridgeDaoPda } = ctx;

    await fund(proposer1.publicKey, provider);

    txn = await program.methods
      .addValidAddresses([proposer1.publicKey])
      .accounts({
        adder: authority.publicKey,
        fridgeDao: fridgeDaoPda,
      })
      .rpc();

    let dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    const nextProposalId = dao.proposalCount.toNumber() + 1;

    const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("fridge_prop"),
        fridgeDaoPda.toBuffer(),
        new anchor.BN(nextProposalId).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    txn = await program.methods
      .propose("Soda")
      .accounts({
        proposer: proposer1.publicKey,
        fridgeDao: fridgeDaoPda,
        proposal: proposalPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([proposer1])
      .rpc();

    dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    assert(
      dao.proposals.find((a) => a.toBase58() == proposalPda.toBase58()),
      "Proposal not found in DAO"
    );
  });

  it("Can cancel proposals", async () => {
    const { provider, program, authority, fridgeDaoPda } = ctx;

    let dao = await program.account.fridgeDao.fetch(fridgeDaoPda);

    const lastProposalId = dao.proposalCount.toNumber();

    const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("fridge_prop"),
        fridgeDaoPda.toBuffer(),
        new anchor.BN(lastProposalId).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const proposal = await program.account.proposal.fetch(proposalPda);

    const balanceBefore = await provider.connection.getBalance(
      proposer1.publicKey
    );

    txn = await program.methods
      .cancelProposal(new anchor.BN(1))
      .accounts({
        canceller: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        proposal: proposalPda,
        proposer: proposal.proposer,
      })
      .rpc();

    const balanceAfter = await provider.connection.getBalance(
      proposer1.publicKey
    );

    assert(
      balanceAfter > balanceBefore,
      "Rent was not returned to the proposer"
    );
    dao = await program.account.fridgeDao.fetch(fridgeDaoPda);
    assert(!dao.proposals.includes(proposalPda), "Proposal still found in DAO");
  });
});
