import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FrenTree } from "../target/types/fren_tree";

describe("fren_tree", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FrenTree as Program<FrenTree>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
