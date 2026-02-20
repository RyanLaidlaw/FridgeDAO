import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { FridgeDao } from "../target/types/fridge_dao";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

describe("fridge_dao", () => {
  // Configure the client to use the local cluster.
  const provder = anchor.AnchorProvider.env()
  anchor.setProvider(provder);

  const program = anchor.workspace.fridgeDao as Program<FridgeDao>;
  const authority = provder.wallet;

  it("Is correctly initialized", async () => {
      const [fridgeDaoPda, fridgeDaoBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fridge")],
        program.programId
      );

      const [treasuryAuthPda, treasuryAuthBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("treasury_authority"),
          fridgeDaoPda.toBuffer(),
        ],
        program.programId
      );

      const usdcMint = await createMint(
        provder.connection,
        authority.payer,
        authority.publicKey,
        null, // possibly add freeze authority later
        6
      );

      const treasuryTokenAccount = anchor.web3.Keypair.generate();

      const txn = await program.methods.initialize().accounts({
        authority: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        treasuryAuthority: treasuryAuthPda,
        treasury: treasuryTokenAccount.publicKey,
        usdcMint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([treasuryTokenAccount])
      .rpc()

      console.log("Txn signature:", txn);

      const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);
      console.log("FridgeDao:", dao);

      assert.equal(dao.authority.toBase58(), authority.publicKey.toBase58(), "Authorities not equal");
      assert.equal(dao.treasury.toBase58(), treasuryTokenAccount.publicKey.toBase58(), 'Treasuries not equal');
      assert.equal(dao.usdcMint.toBase58(), usdcMint.toBase58(), "USDC Mint accounts not equal");
      assert(dao.proposalCount.eq(new anchor.BN(0)), "Proposal Count is not cleared to 0")
      assert.equal(dao.bump, fridgeDaoBump, "Fridge Bumps not equal");
      assert.equal(dao.treasuryBump, treasuryAuthBump, "Treasury Bumps not equal");
  });
});
