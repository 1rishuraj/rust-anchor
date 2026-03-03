import * as anchor from "@coral-xyz/anchor"
import { YourProgram } from "../target/types/your_program";
import { assert } from "chai";
import { Buffer } from "buffer";
describe("pda_creation",()=>{
    //provider
    anchor.setProvider(anchor.AnchorProvider.env());
    //workspace
    let program=anchor.workspace.yourProgram as anchor.Program<YourProgram>;
    //keypair
    let payer=anchor.getProvider().publicKey;
    
    let [pda_acc,bump]=anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("client1"),
        payer?.toBuffer()!
    ],program.programId)
    //it
    it("checking creation",async()=>{
        let ix=await program.methods.createPdaAccount()
        // @ts-ignore
        .accounts({payer:payer,systemProgram:anchor.web3.SystemProgram.programId})
        .rpc();
        console.log(ix)
        let acc=await program.account.stakingAccount.fetch(pda_acc)
        assert.ok(acc,"msg on error: pda not exists!")

    })
    
})