import { test, expect} from "bun:test";
import { LiteSVM } from "litesvm";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, } from "@solana/web3.js";

test("Create pda from client", () => {
    let liveSvm: LiteSVM;
    let pda: PublicKey;
    let bump: number;
    let programId: PublicKey;
    let payer: Keypair;
    
    liveSvm = new LiteSVM();
    

  // create wallet user as payer+airdrop
    payer = Keypair.generate();
    liveSvm.airdrop(payer.publicKey, BigInt(100000000000));

  // deploy the pda-contract on local solana blockchain
    programId = PublicKey.unique();
    liveSvm.addProgramFromFile(programId, "folder/pda.so");

  // pass the derived pda for checking to contract
  // Note the order of seeds must be same in contract as well
    [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("client1"), payer.publicKey.toBuffer()], programId);

  // instruction
    let ix = new TransactionInstruction({
      keys: [
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        }
      ],
      programId,
      data: Buffer.from("")
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = liveSvm.latestBlockhash();
    tx.sign(payer);
    let res = liveSvm.sendTransaction(tx);
    console.log(res.toString())

    const balance = liveSvm.getBalance(pda);
    console.log(balance)
    expect(Number(balance)).toBeGreaterThan(0);
    expect(Number(balance)).toBe(1000000000);
});

