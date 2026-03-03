import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calc } from "../target/types/calc";
import assert from "assert";

describe("anchor-calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calc as Program<Calc>;
  const newAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
      .accounts({
        newAccount: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is double!", async () => {
    const tx = await program.methods.double()
      .accounts({
        newAccount: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.newAccount.fetch(newAccount.publicKey);
    assert.equal(account.count, 2);
  });

  it("Is halve!", async () => {
    const tx = await program.methods.halve()
      .accounts({
        newAccount: newAccount.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.newAccount.fetch(newAccount.publicKey);
    assert.equal(account.count, 1);
  });

  it("Is added!", async () => {
    const tx = await program.methods.add(2)
      .accounts({
        newAccount: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.newAccount.fetch(newAccount.publicKey);
    assert.equal(account.count, 3);
  });
  
  it("subtract",async()=>{
    const txn=await program.methods.sub(0)
    .accounts({
      newAccount:newAccount.publicKey,
      signer:anchor.getProvider().wallet.publicKey
    })
    .rpc();
    const acc=await program.account.newAccount.fetch(newAccount.publicKey)
    assert.equal(acc.count,3)
  })
  
});
