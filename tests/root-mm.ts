import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RootMm } from "../target/types/root_mm";

describe("root-mm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RootMm as Program<RootMm>;

  it("", async () => {
    // Add your test here.
  });
});
