import {expect, test} from "bun:test"
import {LiteSVM} from "litesvm"
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js"
test("here",()=>{
  //svm
  let svm=new LiteSVM();
  const prog=Keypair.generate();
  svm.addProgramFromFile(prog.publicKey,'folder/counter.so');
  //fund payer //data kp
  let payer=Keypair.generate();
  let data_acc=Keypair.generate();
  svm.airdrop(payer.publicKey,BigInt(LAMPORTS_PER_SOL));

  //call init
  let ix=new Transaction();
  ix.add({
    keys: [
      {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: false
      },
      {
          pubkey: data_acc.publicKey,
          isSigner: true,
          isWritable: true
      },
      {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false
      }
    ],
   
    programId: prog.publicKey,
    data: Buffer.from([0]),
  })
  ix.feePayer=payer.publicKey
  ix.recentBlockhash=svm.latestBlockhash();
  ix.sign(payer,data_acc)
  svm.sendTransaction(ix);
  let innerdata=svm.getAccount(data_acc.publicKey)
  expect(innerdata?.data[0]).toBe(1)
  expect(innerdata?.data[1]).toBe(0)
  expect(innerdata?.data[2]).toBe(0)
  expect(innerdata?.data[3]).toBe(0)
  //call double
  let ix2=new Transaction();
  ix2.add({
           keys: [
             {
                 pubkey: data_acc.publicKey,
                 isSigner: false,
                 isWritable: true
             },
           ],

           programId: prog.publicKey,
           data: Buffer.from([1]),
         })
  ix2.feePayer=payer.publicKey
  ix2.recentBlockhash=svm.latestBlockhash();
  ix2.sign(payer)
  svm.sendTransaction(ix2);
  innerdata=svm.getAccount(data_acc.publicKey)
  expect(innerdata?.data[0]).toBe(2)
  expect(innerdata?.data[1]).toBe(0)
  expect(innerdata?.data[2]).toBe(0)
  expect(innerdata?.data[3]).toBe(0)
  //call half
  let ix3=new Transaction();
  ix3.add({
           keys: [
             {
                 pubkey: data_acc.publicKey,
                 isSigner: false,
                 isWritable: true
             },
           ],

           programId: prog.publicKey,
           data: Buffer.from([2]),
         })
  ix3.feePayer=payer.publicKey
  ix3.recentBlockhash=svm.latestBlockhash();
  ix3.sign(payer)
  svm.sendTransaction(ix3);
  innerdata=svm.getAccount(data_acc.publicKey)
  expect(innerdata?.data[0]).toBe(1)
  expect(innerdata?.data[1]).toBe(0)
  expect(innerdata?.data[2]).toBe(0)
  expect(innerdata?.data[3]).toBe(0)
})