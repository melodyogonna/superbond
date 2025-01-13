import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Superbond } from "../target/types/superbond";

describe("superbond", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet;
  const program = anchor.workspace.Superbond as Program<Superbond>;
  const mint = anchor.web3.Keypair.generate();

  it("Token is created", async () => {
    try {
      const bondKeyPair = anchor.web3.Keypair.generate();
      const tx = await program.methods
        .createToken("usd", "$", "", bondKeyPair.publicKey)
        .accounts({
          signer: payer.publicKey,
          mint: mint.publicKey,
        })
        .signers([mint])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      if (e instanceof anchor.AnchorError) {
        console.error(e.error.errorMessage);
      } else if (e instanceof anchor.web3.SendTransactionError) {
        console.log(e.transactionError);
        console.error(e.getLogs(provider.connection));
      } else {
        console.error("Unexpected error", e);
      }
      throw e;
    }
  });
});
