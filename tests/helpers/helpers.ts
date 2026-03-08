import * as anchor from "@coral-xyz/anchor";

export async function fund(acct: anchor.web3.PublicKey, provider: anchor.AnchorProvider) {
    const sig = await provider.connection.requestAirdrop(acct, 2 * anchor.web3.LAMPORTS_PER_SOL);

    await provider.connection.confirmTransaction(sig);
}