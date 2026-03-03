import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import { Transfercpi } from "../target/types/transfercpi";
import { BN } from "bn.js";
describe("check",()=>{
  //setprovider
  anchor.setProvider(anchor.AnchorProvider.env())
  //get program object from workspace
  let program=anchor.workspace.transfercpi as anchor.Program<Transfercpi>;
  //generate keypairs
  let payer=anchor.getProvider().publicKey;
  let recipient=anchor.web3.Keypair.generate();
  //it - test fxn
  it("transfer",async()=>{
    //program methods
    let ix= await program.methods.soltransfer(new BN(1000000000)).accounts({
      signer:payer,
      recipient:recipient.publicKey
    }).rpc();
    let acc=anchor.getProvider().connection.getAccountInfo(recipient.publicKey)
    //assert
    assert.equal((await acc).lamports,1000000000)
  })
  
})