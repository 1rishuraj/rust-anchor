import { Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import {test,expect} from "bun:test"
import { LiteSVM } from "litesvm"
test("doubling count",()=>{
    const svm=new LiteSVM();
    const contractKeypair=Keypair.generate();
    svm.addProgramFromFile(contractKeypair.publicKey,'./double.so')
    const payer=Keypair.generate();
    const dataAccount=Keypair.generate();
    const ix=[
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: dataAccount.publicKey,
            lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),//sol needed for 4 bytes storage
            space: 4 , //as count:u32 = 32 bits = 4 bytes
            programId: contractKeypair.publicKey
        })
    ]
    const tx=new Transaction();
    tx.feePayer=payer.publicKey;
    tx.recentBlockhash=svm.latestBlockhash();
    tx.add(...ix);
    tx.sign(payer,dataAccount);
    svm.sendTransaction(tx);
    const bal_after=svm.getBalance(dataAccount.publicKey);
    expect(bal_after).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));
    //bal of dataacc must be same as rentSol
})