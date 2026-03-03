import * as anchor from "@coral-xyz/anchor";
import { TransferContract } from "../target/types/transferContract";
import { BN } from "bn.js";
import { assert } from "chai";

describe("lets see",()=>{
  //set provider
  anchor.setProvider(anchor.AnchorProvider.env());
  //set workplace program
  let program=anchor.workspace.transferContract as anchor.Program<TransferContract>;
  //keypairs
  let sender = anchor.getProvider().publicKey;
  let recipient=anchor.web3.Keypair.generate();
  let sys_program=anchor.web3.SystemProgram.programId;
  //it instruction
  it("transfer amt",async ()=>{
    //prog methods
    let res= await program.methods.soltranfer(new BN(1000000000)).
    accounts({signer:sender,recipient:recipient.publicKey}).rpc()
  //assert
  let acc=await anchor.getProvider().connection.getAccountInfo(recipient.publicKey)
  assert.equal(acc.lamports,1000000000);
  })
  
})