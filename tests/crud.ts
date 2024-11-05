import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crud } from "../target/types/crud";
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
const crudAddress = new PublicKey(
  "Mx2KcqEtge1DbkVTNC3oyM1Bmbvd8eXfADUWxegdvGy"
);
const IDL = require("../target/idl/crud.json");
describe("crud", () => {
  let context;
  let provider;
  let crudProgram;

  before(async () => {
    context = await startAnchor(
      "",
      [{ name: "crud", programId: crudAddress }],
      []
    );
    provider = new BankrunProvider(context);
    crudProgram = new Program<Crud>(IDL, provider);
  });

  it("create journal", async () => {
    await crudProgram.methods
      .createJournalState(
        "First Day",
        "What is your favourite type of peanut butter"
      )
      .rpc();
    // const [journalAddress] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("First Day")],
    //   crudAddress
    // );
  });
});
