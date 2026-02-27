import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { FridgeDao } from "../target/types/fridge_dao";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

describe("fridge_dao", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.fridgeDao as Program<FridgeDao>;
  const authority = provider.wallet;

  it("Is correctly initialized", async () => {
      const [fridgeDaoPda, fridgeDaoBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fridge")],
        program.programId
      );

      const [vaultAuthPda, vaultAuthBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault_authority"),
          fridgeDaoPda.toBuffer(),
        ],
        program.programId
      );

      const usdcMint = await createMint(
        provider.connection,
        authority.payer,
        authority.publicKey,
        null, // possibly add freeze authority later
        6
      );

      const vaultTokenAccount = anchor.web3.Keypair.generate();

      const voting_period = 1000;
      const time_until_start = 100;
      const slot = await provider.connection.getSlot();
      const blockTime = await provider.connection.getBlockTime(slot);

      const txn = await program.methods.initialize(new anchor.BN(voting_period), new anchor.BN(time_until_start)).accounts({
        authority: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        vaultAuthority: vaultAuthPda,
        vault: vaultTokenAccount.publicKey,
        usdcMint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vaultTokenAccount])
      .rpc()

      console.log("Txn signature:", txn);

      const dao = await program.account.fridgeDao.fetch(fridgeDaoPda);
      console.log("FridgeDao:", dao);

      assert.equal(dao.authority.toBase58(), authority.publicKey.toBase58(), "Authorities not equal");
      assert.equal(dao.vault.toBase58(), vaultTokenAccount.publicKey.toBase58(), 'Treasuries not equal');
      assert.equal(dao.usdcMint.toBase58(), usdcMint.toBase58(), "USDC Mint accounts not equal");
      assert(dao.proposalCount.eq(new anchor.BN(0)), "Proposal Count is not cleared to 0")
      assert.equal(dao.bump, fridgeDaoBump, "Fridge Bumps not equal");
      assert.equal(dao.vaultBump, vaultAuthBump, "Vault Bumps not equal");

      assert(dao.votePeriod.eq(new anchor.BN(voting_period)), "Vote periods not equal");
      assert(new anchor.BN(blockTime).lte(dao.nextVoteAllowedAt),"We are not past the voting start time");
  });
});
