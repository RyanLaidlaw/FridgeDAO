import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FridgeDao } from "../../target/types/fridge_dao";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

export async function setup() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.fridgeDao as Program<FridgeDao>;
  const authority = provider.wallet;

  let fridgeDaoPda: anchor.web3.PublicKey;
  let fridgeDaoBump: number;

  let vaultAuthPda: anchor.web3.PublicKey;
  let vaultAuthBump: number;

  let usdcMint: anchor.web3.PublicKey;
  let vaultTokenAccount: anchor.web3.Keypair;

  let votingPeriod = 1000;
  let timeUntilStart = 100;

  let blockTime: number | null;
  let txn: string;

  [fridgeDaoPda, fridgeDaoBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("fridge")],
    program.programId
  );

  [vaultAuthPda, vaultAuthBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_authority"), fridgeDaoPda.toBuffer()],
    program.programId
  );

  usdcMint = await createMint(
    provider.connection,
    authority.payer,
    authority.publicKey,
    null,
    6
  );

  vaultTokenAccount = anchor.web3.Keypair.generate();

  const slot = await provider.connection.getSlot();
  blockTime = await provider.connection.getBlockTime(slot);

  const acct = await provider.connection.getAccountInfo(fridgeDaoPda);

  if (!acct) {
    txn = await program.methods
      .initialize(new anchor.BN(votingPeriod), new anchor.BN(timeUntilStart))
      .accounts({
        authority: authority.publicKey,
        fridgeDao: fridgeDaoPda,
        vaultAuthority: vaultAuthPda,
        vault: vaultTokenAccount.publicKey,
        usdcMint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vaultTokenAccount])
      .rpc();
  }

  return {
    provider,
    program,
    authority,
    usdcMint,
    fridgeDaoPda,
    fridgeDaoBump,
    vaultTokenAccount,
    vaultAuthPda,
    vaultAuthBump,
    blockTime,
    votingPeriod,
  };
}
