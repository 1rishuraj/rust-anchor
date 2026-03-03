import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { expect, test } from "bun:test";
import { Counter_Size, CounterAccount, schema } from "./types";
import * as borsh from "borsh";
import bs58 from "bs58";
const privateKeyHex =
  "zXs6ABCDzYbZzcWL"; // Replace with your key

const privateKeyBytes = bs58.decode(privateKeyHex);

const uint8Array = new Uint8Array(privateKeyBytes);
let walletAcc = Keypair.fromSecretKey(uint8Array);
let dataAcc = Keypair.generate();

const programID = new PublicKey("BN8r82ccwZiFUu4kkBiw2ViAjpvTxvYHYcZR5FRHacwX");
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

test("Account is initialised", async () => {
  const txn = await connection.requestAirdrop(
    walletAcc.publicKey,
    1 * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(txn); // Critical: wait for confirmation
  console.log("Airdrop confirmed:", txn);
  const data = await connection.getAccountInfo(walletAcc.publicKey);
  //airdrop done

  const lamports = await connection.getMinimumBalanceForRentExemption(
    Counter_Size
  );
  const ix = SystemProgram.createAccount({
    fromPubkey: walletAcc.publicKey,
    lamports,
    space: Counter_Size,
    programId: programID,
    newAccountPubkey: dataAcc.publicKey,
  });
  const createDataAccTxn = new Transaction();
  createDataAccTxn.add(ix);
  const sign = await connection.sendTransaction(createDataAccTxn, [
    walletAcc,
    dataAcc,
  ]);
  console.log("Data account created:", dataAcc.publicKey.toBase58());

  console.log("Counter_Size:", Counter_Size);

  const tx = new Transaction();
  tx.add(
    new TransactionInstruction({
      keys: [
        {
          pubkey: dataAcc.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ],
      programId: programID,
      data: Buffer.from(new Uint8Array([0, 1, 0, 0, 0])),
    })
  );

  // // Send and confirm the transaction
  const txHash = await connection.sendTransaction(tx, [walletAcc, dataAcc]);
  await connection.confirmTransaction(txHash);
  console.log(txHash);
  // valid case: real serialized data
  const acccInfo = await connection.getAccountInfo(dataAcc.publicKey);
  const counter = borsh.deserialize(schema, CounterAccount, acccInfo!.data);
  console.log("Counter after increment:", counter.count);
  expect(counter.count).toBe(1);
});
