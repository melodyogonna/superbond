import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Superbond } from "../target/types/superbond";

describe("superbond", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const payer = provider.wallet;
  const program = anchor.workspace.Superbond as Program<Superbond>;

  it("Token is created", async () => {
    // Add your test here.
    const bondKeyPair = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .createToken("usd", "$", "", bondKeyPair.publicKey)
      .accounts({
        signer: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
