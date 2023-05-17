// Rust Client
import * as web3 from "@solana/web3.js"
import { initializeSolSignerKeypair, airdropSolIfNeeded } from "./initializeKeypair" // for creating and saving userKeypair as well as airdrop SOL if needed
import * as borsh from "borsh" // for serialization and deserialization

// Define program id to greet
const PROGRAM_ID = new web3.PublicKey("3v5s9Jf3AHpoxCieQCEsiD8FYe9yEkZcZ6wVvCcsm4g4")

// call greet()
greet().then(() => {
  console.log('Finished successfully')
  process.exit(0)
}).catch(error => {
  console.log(error)
  process.exit(1)
})


async function greet() {

  // Set up a connection to blockchain cluster
  const cluster = 'devnet'
  const connection = new web3.Connection(web3.clusterApiUrl(cluster), 'confirmed')

  // Create user keypair and airdrop SOL if needed
  const userKeypair = initializeSolSignerKeypair()
  await airdropSolIfNeeded(userKeypair, connection)


  const lamports = await connection.getMinimumBalanceForRentExemption(
    GREETING_SIZE
  );

  let [greetingPda, bump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("greeting_account"), userKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // Create greet instruction
  const greetIx = new web3.TransactionInstruction({
    keys: [
      {
        pubkey: userKeypair.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: greetingPda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      }
    ],
    programId: PROGRAM_ID,
  });

  // Create transaction and add the instructions
  const tx = new web3.Transaction();
  tx.add(
    // createGreetingAccountIx,
    greetIx
  );

  // Send and confirm the transaction
  console.log(`Sending transaction with commitment level '${connection.commitment}' to program ${PROGRAM_ID} ...`);
  const txHash = await web3.sendAndConfirmTransaction(connection, tx, [
    userKeypair,
    // greetingAccountKp,
  ]);
  console.log(`Use 'solana confirm -v ${txHash}' to see the logs \n`);

  // Fetch the greetings account
  const greetingAccount = await connection.getAccountInfo(
    greetingPda
  );

  // Deserialize the account data
  const deserializedAccountData = borsh.deserialize(
    GreetingSchema,
    GreetingAccount,
    greetingAccount!.data
  );


 

  console.log(
    `On-chain account ${greetingPda}'s data field 'counter': ${deserializedAccountData.counter} \n`
  );
}



/**
 *  State account structure
 */
class GreetingAccount {
    counter = 0;
    constructor(fields: { counter: number } | undefined = undefined) {
      if (fields) {
        this.counter = fields.counter;
      }
    }
  }
  
  /**
   * Borsh schema definition for greeting state account
   */
  const GreetingSchema = new Map([
    [GreetingAccount, { kind: "struct", fields: [["counter", "u32"]] }],
  ]);
  
  /**
   * The expected size of each greeting account.
   */
  const GREETING_SIZE = borsh.serialize(
    GreetingSchema,
    new GreetingAccount()
  ).length;