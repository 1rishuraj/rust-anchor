import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import {test,expect} from "bun:test";
import { LiteSVM } from "litesvm";
test("invoking Double via cpi",()=>{
  //deploy cpi + double contract on chain
  const svm = LiteSVM.default();
  const cpiKeypair=PublicKey.unique();
  const doubleKeypair=PublicKey.unique();
  svm.addProgramFromFile(cpiKeypair,'folder/cpi.so');
  svm.addProgramFromFile(doubleKeypair,'folder/double.so');
  
  //create user wallet + airdrop
  const userKeypair=Keypair.generate();
  svm.airdrop(userKeypair.publicKey,BigInt(LAMPORTS_PER_SOL));
  //create data acc
  const data_acc=Keypair.generate();
  create_data_acc(userKeypair,data_acc,svm,doubleKeypair);//doubleKeypair as owner of dataAcc

  //sending cpi contract instruction
  function double_viaCPI(){
    const ix=new TransactionInstruction({
        keys:[
          {pubkey:data_acc.publicKey,isSigner:false,isWritable:true},
          {pubkey:doubleKeypair,isSigner:false,isWritable:false},
          
        ],
        programId:cpiKeypair,
        data:Buffer.from("")
      })
    
    const tx=new Transaction();
    tx.feePayer=userKeypair.publicKey;
    tx.recentBlockhash=svm.latestBlockhash();
    tx.add(ix);
    tx.sign(userKeypair);
    const res=svm.sendTransaction(tx);
    console.log(res.toString())
    svm.expireBlockhash();
  }
  double_viaCPI();
  double_viaCPI();
  double_viaCPI();
  double_viaCPI();
  //check
  const data_account=svm.getAccount(data_acc.publicKey);
  console.log(data_account?.data)
  expect(data_account?.data[0]).toBe(8);
  expect(data_account?.data[1]).toBe(0);
  expect(data_account?.data[2]).toBe(0);
  expect(data_account?.data[3]).toBe(0);
})

function create_data_acc(payer:Keypair,dataAccount:Keypair,svm:LiteSVM,contractKeypair:PublicKey){
  const ix=[
    SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: dataAccount.publicKey,
        lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),//sol needed for 4 bytes storage
        space: 4 , //as count:u32 = 32 bits = 4 bytes
        programId: contractKeypair
    })
  ]
  const tx=new Transaction();
  tx.feePayer=payer.publicKey;
  tx.recentBlockhash=svm.latestBlockhash();
  tx.add(...ix);
  tx.sign(payer,dataAccount);
  svm.sendTransaction(tx);
}