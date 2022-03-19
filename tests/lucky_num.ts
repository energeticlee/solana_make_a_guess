import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
// import { LuckyNum } from "../target/types/lucky_num"; <-- causes error
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
const assert = require("assert");

describe("lucky_num", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  // const program = anchor.workspace.LuckyNum as Program<LuckyNum>;
  const program = anchor.workspace.LuckyNum;

  const personA = anchor.web3.Keypair.generate();
  const personB = anchor.web3.Keypair.generate();
  const personC = anchor.web3.Keypair.generate();
  const personD = anchor.web3.Keypair.generate();

  const vault = anchor.web3.Keypair.generate();
  const gameInfo = anchor.web3.Keypair.generate();

  const payer = anchor.web3.Keypair.generate();

  it("Person A and Person B wallets created and funded!", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 90000000000),
      "confirmed"
    );

    // console.log("Payer PK Generated", payer.publicKey.toString()); //Kev
    // console.log(
    //   "--> Payer Balance",
    //   await provider.connection.getBalance(payer.publicKey)
    // );

    await provider.send(
      //provider.send 1st param is a tx, 2nd param is signer
      (() => {
        const tx = new Transaction();
        tx.add(
          //   multiple transactions separated by commas
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: personA.publicKey,
            lamports: 10000000000, //10 sol
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: personB.publicKey,
            lamports: 10000000000, //10 sol
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: personC.publicKey,
            lamports: 10000000000, //10 sol
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: personD.publicKey,
            lamports: 10000000000, //10 sol
          })
        );
        return tx;
      })(),
      [payer]
    );

    // console.log("personA Wallet PK: ", personA.publicKey.toString());
    // console.log(
    //   "personA Balance",
    //   await provider.connection.getBalance(personA.publicKey)
    // );
    // console.log("personB Wallet PK: ", personB.publicKey.toString());
    // console.log(
    //   "personB Balance",
    //   await provider.connection.getBalance(personB.publicKey)
    // );
    // console.log("personC Wallet PK: ", personC.publicKey.toString());
    // console.log(
    //   "personC Balance",
    //   await provider.connection.getBalance(personC.publicKey)
    // );
    // console.log("personD Wallet PK: ", personD.publicKey.toString());
    // console.log(
    //   "personD Balance",
    //   await provider.connection.getBalance(personD.publicKey)
    // );
  });

  it("Person A Joining Game", async () => {
    // console.log(
    //   `personA Get balance PRE: ${
    //     (await program.provider.connection.getBalance(personA.publicKey)) /
    //     LAMPORTS_PER_SOL
    //   } SOL`
    // );
    const [userStatsPDA, userStatsBump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("pubkey")],
      program.programId
    );

    await program.rpc.initialize(
      new anchor.BN(1 * LAMPORTS_PER_SOL), // stake - 1 sol
      new anchor.BN(3), // max participants
      new anchor.BN(1), // lucky_num
      userStatsBump,
      {
        accounts: {
          gameInfo: gameInfo.publicKey,
          vault: userStatsPDA,
          initializer: personA.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [personA, gameInfo],
      }
    );
    // const account = await program.account.vault.fetch(userStatsPDA);
    // console.log("accountPostInit ", account.amount.toString());
    // console.log(
    //   `Vault Get balance A: ${
    //     (await program.provider.connection.getBalance(userStatsPDA)) /
    //     LAMPORTS_PER_SOL
    //   } SOL`
    // );
    // console.log(
    //   `personA Get balance POST: ${
    //     (await program.provider.connection.getBalance(personA.publicKey)) /
    //     LAMPORTS_PER_SOL
    //   } SOL`
    // );

    // const accountInfoAfterPA = await program.provider.connection.getAccountInfo(
    //   vault.publicKey
    // );
    // console.log("accountInfoAfterPA", accountInfoAfterPA.lamports);
    // console.log("+account.amount", +account.amount);
    // console.log("LAMPORTS_PER_SOL", LAMPORTS_PER_SOL);

    // assert(+account.amount === LAMPORTS_PER_SOL, "Vault amount incorrect");
    //? Question 1: Why is it possible to pass a public key into gameInfo, does not not expect a Game struct?
    //? Question: Why is gameInfo required as a signer?
  });

  it("Person B Joining Game", async () => {
    const [userStatsPDA, _] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("pubkey")],
      program.programId
    );

    // const gameAccountInfo = await program.provider.connection.getAccountInfo(
    //   gameInfo.publicKey
    // );
    // const vaultAccountInfo = await program.provider.connection.getAccountInfo(
    //   userStatsPDA
    // );
    // console.log("vaultAccountInfo", vaultAccountInfo);

    await program.rpc.participate(
      new anchor.BN(1 * LAMPORTS_PER_SOL), // stake - 1 sol
      new anchor.BN(2), // lucky_num - 1
      {
        accounts: {
          participantAccount: personB.publicKey,
          vault: userStatsPDA,
          gameInfo: gameInfo.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [personB],
        //? Question 2: Why gameInfo not required to be signer eventhough we are mutating state
      }
    );
  });

  it("Person C Joining Game", async () => {
    const [userStatsPDA, _] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("pubkey")],
      program.programId
    );
    // const accounta = await program.account.vault.fetch(userStatsPDA);
    // console.log("accountPreCJoining ", accounta.amount.toString());

    await program.rpc.participate(
      new anchor.BN(1 * LAMPORTS_PER_SOL), // stake - 1 sol
      new anchor.BN(3), // lucky_num - 6
      {
        accounts: {
          participantAccount: personC.publicKey,
          vault: userStatsPDA,
          gameInfo: gameInfo.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [personC],
      }
    );
    // const account = await program.account.vault.fetch(userStatsPDA);
    // console.log("accountPostCJoining ", account.amount.toString());
  });

  it("Exchange", async () => {
    const [userStatsPDA, _] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("pubkey")],
      program.programId
    );
    // let gameAccountInfo = await program.account.game.fetch(gameInfo.publicKey);
    // console.log(
    //   "gameAccountInfo.maxParticipants",
    //   gameAccountInfo.maxParticipants
    // );
    // console.log(
    //   "gameAccountInfo.participantList",
    //   gameAccountInfo.participantList
    // );
    // const particpantArr = await Promise.all(
    //   gameAccountInfo.participantList.map(async (acc) => {
    //     return await program.provider.connection.getAccountInfo(
    //       acc.participantAddress
    //     );
    //   })
    // );
    // const playerOne = await program.provider.connection.getAccountInfo(
    //   personA.publicKey
    // );
    // const playerTwo = await program.provider.connection.getAccountInfo(
    //   personB.publicKey
    // );
    // const playerThree = await program.provider.connection.getAccountInfo(
    //   personC.publicKey
    // );
    // const userStatsPDAInfo = await program.provider.connection.getAccountInfo(
    //   userStatsPDA
    // );

    // console.log("playerOne ", playerOne);
    // console.log("playerTwo ", playerTwo);
    // console.log("playerThree ", playerThree);
    // console.log("playerThree ", userStatsPDAInfo);
    // console.log("gameAccountInfo ", gameAccountInfo);
    // const accounta = await program.account.vault.fetch(userStatsPDA);
    // console.log("accountPreExchange ", accounta.amount.toString());

    // console.log(
    //   "particpantArr[0].accountInfo.key()",
    //   particpantArr[0].accountInfo.key()

    //   //the rust way -  to_account_info().key
    // );
    const playerOneA1 = await program.provider.connection.getAccountInfo(
      personA.publicKey
    );
    console.log("playerOne ", playerOneA1.lamports);

    await program.rpc.exchange({
      accounts: {
        playerOne: personA.publicKey,
        playerTwo: personB.publicKey,
        playerThree: personC.publicKey,
        gameInfo: gameInfo.publicKey,
        vault: userStatsPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      // signers: [personC],
      //? Question 2: Why gameInfo not required to be signer eventhough we are mutating state
      //? Question: Passing a public key into an Account that expects AccountInfo work? see program.rpc.exchange above
    });

    const playerOneA = await program.provider.connection.getAccountInfo(
      personA.publicKey
    );
    const playerTwoB = await program.provider.connection.getAccountInfo(
      personB.publicKey
    );
    const playerThreeC = await program.provider.connection.getAccountInfo(
      personC.publicKey
    );

    const userStatsPDAInfoA = await program.provider.connection.getAccountInfo(
      userStatsPDA
    );

    console.log("playerOne ", playerOneA.lamports);
    console.log("playerTwo ", playerTwoB.lamports);
    console.log("playerThree ", playerThreeC.lamports);
    // const account = await program.account.vault.fetch(userStatsPDA);
    // console.log("accountPostExchange ", account.amount.toString());
    // console.log("gameAccountInfo ", gameAccountInfo);

    // console.log("personA Wallet PK: ", personA.publicKey.toString());
    // console.log(
    //   "personA Balance",
    //   await provider.connection.getBalance(personA.publicKey)
    // );
    // console.log("personB Wallet PK: ", personB.publicKey.toString());
    // console.log(
    //   "personB Balance",
    //   await provider.connection.getBalance(personB.publicKey)
    // );
    // console.log("personC Wallet PK: ", personC.publicKey.toString());
    // console.log(
    //   "personC Balance",
    //   await provider.connection.getBalance(personC.publicKey)
    // );
  });
  // console.log("accountInfoAfterPB", accountInfoAfterPB.lamports);
  // console.log(
  //   `Vault Get balance after B: ${
  //     (await program.provider.connection.getBalance(vault.publicKey)) /
  //     LAMPORTS_PER_SOL
  //   } SOL`
  // );
  // console.log(
  //   `personB Get balance B: ${
  //     (await program.provider.connection.getBalance(personB.publicKey)) /
  //     LAMPORTS_PER_SOL
  //   } SOL`
  // );

  // it("Check if num player == 2", async () => {
  //   let accounts = await program.account.game.all();
  //   const filteredList = accounts.filter(
  //     (acc) =>
  //       acc.account.initializerKey.toString() === personA.publicKey.toString()
  //   );
  //   assert(
  //     filteredList[0].account.participantList.length === 2,
  //     "max participants reached"
  //   );
  // });

  // it("Person C Joining Game", async () => {
  //   const [userStatsPDA, _] = await PublicKey.findProgramAddress(
  //     [anchor.utils.bytes.utf8.encode("pubkey")],
  //     program.programId
  //   );

  //   await program.rpc.participate(
  //     new anchor.BN(1 * LAMPORTS_PER_SOL), // stake - 1 sol
  //     new anchor.BN(3), // lucky_num - 6
  //     {
  //       accounts: {
  //         participantAccount: personC.publicKey,
  //         vault: userStatsPDA,
  //         gameInfo: gameInfo.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       },
  //       signers: [personC],
  //     }
  //   );
  // });

  // it("Should have 3 players", async () => {
  //   let accounts = await program.account.game.all();
  //   const filteredList = accounts.filter(
  //     (acc) =>
  //       acc.account.initializerKey.toString() === personA.publicKey.toString()
  //   );
  //   assert(
  //     filteredList[0].account.maxParticipants ==
  //       filteredList[0].account.participantList.length,
  //     "max participants reached"
  //   );
  // });

  // it("Person D Joining Game, should fail", async () => {
  //   try {
  //     const [userStatsPDA, _] = await PublicKey.findProgramAddress(
  //       [anchor.utils.bytes.utf8.encode("pubkey")],
  //       program.programId
  //     );
  //     await program.rpc.participate(
  //       new anchor.BN(1 * LAMPORTS_PER_SOL), // stake - 1 sol
  //       new anchor.BN(4), // lucky_num - 6
  //       {
  //         accounts: {
  //           participantAccount: personD.publicKey,
  //           vault: userStatsPDA,
  //           gameInfo: gameInfo.publicKey,
  //           systemProgram: anchor.web3.SystemProgram.programId,
  //         },
  //         signers: [personD],
  //       }
  //     );
  //     assert(false);
  //   } catch {
  //     assert(true);
  //   }
  // });

  // it("Player A and B should be Winner?", async () => {
  //   let accounts = await program.account.game.all();
  //   const filteredList = accounts.filter(
  //     (acc) =>
  //       acc.account.initializerKey.toString() === personA.publicKey.toString()
  //   );
  //   assert(
  //     filteredList[0].account.maxParticipants <
  //       filteredList[0].account.participantList.length,
  //     "max participants reached"
  //   );
  // });
});

//const account = await program.account.baseAccount.fetch(baseAccount.publicKey);

// acc.account.initializerKey
// personA.publicKey.toString() = 8rsmDfpR8xqzMseTmJSdRkmZxwYhZid6fQByjvzxq6fv
// <BN: d5df06cb0f226b0cd7522d02fb14717d422ea814832abb99c8b138e82ce1b32e>
// <BN: 9490c217aac901f64530922e85ede47a54742cdb4187b708094ccb3fec2f6a90>
