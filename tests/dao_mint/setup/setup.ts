import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaoMint } from "../../../target/types/dao_mint";
import { FridgeDao } from "../../../target/types/fridge_dao";
import { 
    TOKEN_PROGRAM_ID, 
    createMint, 
    getOrCreateAssociatedTokenAccount,
    mintTo
} from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";

export async function setup() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const daoMintProgram = anchor.workspace.DaoMint as Program<DaoMint>;
    const fridgeDaoProgram = anchor.workspace.FridgeDao as Program<FridgeDao>;
    const authority = provider.wallet;

    const [daoMintPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("fridge_mint")],
        daoMintProgram.programId
    );

    const [fridgeDaoPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("fridge")],
        fridgeDaoProgram.programId
    );

    let usdcMint = await createMint(
        provider.connection,
        authority.payer,
        authority.publicKey,
        null,
        6
    );

    const adminTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority.payer,
        usdcMint,
        authority.publicKey
    );

    await mintTo(
        provider.connection,
        authority.payer,
        usdcMint,
        adminTokenAccount.address,
        authority.payer,
        1_000_000_000
    );

    const acct = await provider.connection.getAccountInfo(daoMintPda);

    let vault: PublicKey;

    if (!acct) {
        const vaultKeypair = Keypair.generate();

        await daoMintProgram.methods
            .initialize(
                new anchor.BN(1000),
                new anchor.BN(100),
            )
            .accounts({
                admin: authority.publicKey,
                daoMint: daoMintPda,
                vault: vaultKeypair.publicKey,
                usdcMint: usdcMint,
                daoProgram: fridgeDaoProgram.programId,
                fridgeDao: fridgeDaoPda,
                adminTokenAccount: adminTokenAccount.address,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([vaultKeypair])
            .rpc();

        vault = vaultKeypair.publicKey;
    } else {
        const daoMintAcct = await daoMintProgram.account.daoMint.fetch(daoMintPda);
        vault = daoMintAcct.vault;
        usdcMint = daoMintAcct.usdcMint;
    }

    return {
        provider,
        daoMintProgram,
        fridgeDaoProgram,
        authority,
        usdcMint,
        daoMintPda,
        fridgeDaoPda,
        vault,
        adminTokenAccount: adminTokenAccount.address,
    };
}