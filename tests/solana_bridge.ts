import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaBridge } from "../target/types/solana_bridge";
import {
  createAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { sendAndConfirmTransaction, Transaction } from '@solana/web3.js';
import type { ConfirmOptions, Connection, PublicKey, Signer } from '@solana/web3.js';
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

// 为不在曲线上的PDA程序创建对应的ATA账户
export async function createPdaAssociatedTokenAccount(
  connection: Connection,
  payer: Signer,
  mint: PublicKey,
  owner: PublicKey,
  confirmOptions?: ConfirmOptions,
  programId = TOKEN_PROGRAM_ID,
  associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID
): Promise<PublicKey> {
  const associatedToken = getAssociatedTokenAddressSync(mint, owner, true, programId, associatedTokenProgramId);

  const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
          payer.publicKey,
          associatedToken,
          owner,
          mint,
          programId,
          associatedTokenProgramId
      )
  );

  await sendAndConfirmTransaction(connection, transaction, [payer], confirmOptions);

  return associatedToken;
}

describe("solana_bridge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const mintpk = new anchor.web3.PublicKey("6MkRWmqMwimvbxtuUDEhUL5rKzkVbAMVbuysm2zYGBf6"); // token
  // const mintpk = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112"); // wsol
  const program = anchor.workspace.SolanaBridge as Program<SolanaBridge>;
  // 初始化stroage的PDA账户内容，设置owner
  // it("Is initialized!", async () => {
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
  //   console.log("the storage account address is", myStorage.toBase58());

  //   await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
  // });

  // it("deleted mystorage!", async () => {
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
  //   console.log("the storage account address is", myStorage.toBase58());

  //   let tx = await program.methods.deleteMystorage().accounts({ myStorage: myStorage }).rpc();
  //   console.log("Your transaction signature", tx);
  // });


  // // 修改storage中的owner
  // it("modify owner!", async () => {
  //   // Add your test here.
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
  //   console.log("the storage account address is", myStorage.toBase58());

  //   let from = new anchor.web3.PublicKey("HDXAyUNpESzg8EkJPojmEeNELTQS6BcVmpZ3pR5DyAfo"); 
  //   // let from = new anchor.web3.PublicKey("2nhbqSk4RvZE7jsq3UZmigtmhSVSVDqrWuWN4ViStVZQ"); 

  //   const txset = await program.methods.modifyOwner(from).accounts({myStorage:myStorage}).rpc();
  //   console.log("Your transaction signature", txset);
  // });

  it("get owner!", async () => {
    // Add your test here.
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
    console.log("the storage account address is", myStorage.toBase58());
    // read the account back
    let result = await program.account.myStorage.fetch(myStorage);
    console.log(`the owner ${result.owner} was stored in ${myStorage.toBase58()}`);
  });

  // 转帐Sol
  it("deposit native to PDA", async () => {
    // Add your test here.
    // let myKeypair = anchor.web3.Keypair.generate();
    const [pragramPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_fund_112")], program.programId);
    console.log("the fundaddress account address is", pragramPDA.toBase58(),"bump", _bump);

    const tx = await program.methods.depositNative(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL), "ETH", "0xdead").accounts({
      // to: "2nhbqSk4RvZE7jsq3UZmigtmhSVSVDqrWuWN4ViStVZQ",
      // to: "DYPjft89CsTpVVaeUW6CX3aBDh7TifjNJ99XavKmK59j",
      to: pragramPDA,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  // 向程序持有的PDA账户装帐Token
  it("deposit Ft to PDA", async () => {
    const [myStorage, _StorageBump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
    console.log("the storage account address is", myStorage.toBase58(),"bump", _StorageBump);

    // const mintpk = new anchor.web3.PublicKey("6MkRWmqMwimvbxtuUDEhUL5rKzkVbAMVbuysm2zYGBf6");

    let from = new anchor.web3.PublicKey("HDXAyUNpESzg8EkJPojmEeNELTQS6BcVmpZ3pR5DyAfo");  
    // let to = new anchor.web3.PublicKey("2UurTVpar9Npe4b1FXNgcVGmowBT3DWJxRX5LSMdjNYq"); 

    let fromAta = await getAssociatedTokenAddress(mintpk, from);
    console.log(`from ATA addr: ${fromAta.toString()}`);

    const [bridgeFund_Wrong, _bum_Wrong] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_fund_Wrong")], program.programId);
    console.log("the pragramPDA account address is", bridgeFund_Wrong.toBase58(),"bump", _bum_Wrong);

    const [pragramPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_fund_112")], program.programId);
    console.log("the pragramPDA account address is", pragramPDA.toBase58(),"bump", _bump);
    let to = pragramPDA;
    let toAta = await getAssociatedTokenAddress(mintpk, to, true);
    console.log(`to ATA addr: ${toAta.toString()}`);

    try { // 如果不存在to这个接受账户，则要为接受地址创建ATA账户
      const tokenAccount = await program.provider.connection.getAccountInfo(toAta);
      // console.log(tokenAccount);

      if (!tokenAccount) {
          const data = new Uint8Array([211,223,118,227,3,217,119,209,86,209,126,149,202,190,149,20,100,84,118,187,106,48,238,210,106,40,175,86,222,52,251,3,240,240,19,121,149,191,246,59,226,56,139,80,209,182,156,66,185,97,0,57,73,239,169,238,16,190,106,46,237,182,44,142]);
          const payer = anchor.web3.Keypair.fromSecretKey(data);
          await createPdaAssociatedTokenAccount(
            program.provider.connection,
            payer,
            mintpk,
            pragramPDA
        );
      }
    } catch (error) {
      console.error("Error checking ATA:", error);
      return;
    }

    const tx = await program.methods.depositFt(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL), "ETH", "0xdead").accounts({
      from: from,
      to: to,
      pda: pragramPDA,
      fromAta: fromAta,
      toAta: toAta,
      mint: mintpk,
      authority: program.provider.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("withdraw native",async() =>{
    const [myStorage, _StorageBump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
    console.log("the storage account address is", myStorage.toBase58(),"bump", _StorageBump);

    // let keySeed = myStorage.toBytes;
    // const [pragramPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync([keySeed], program.programId);
    const [pragramPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_fund_112")], program.programId);
    console.log("the pragramPDA account address is", pragramPDA.toBase58(),"bump", _bump);

    // read the account back
    let result = await program.account.myStorage.fetch(myStorage);
    console.log(`the owner ${result.owner} was stored in ${myStorage.toBase58()}`);

    const tx = await program.methods.withdrawNative("bridge_fund_112", new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
      myStorage:myStorage,
      pda:pragramPDA,
    }).rpc();
    console.log("Your transaction signature", tx);
  })

  it("withdraw Ft",async() =>{
    // const mintpk = new anchor.web3.PublicKey("6MkRWmqMwimvbxtuUDEhUL5rKzkVbAMVbuysm2zYGBf6");

    const [myStorage, _StorageBump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_storage")], program.programId);
    console.log("the storage PDA address is", myStorage.toBase58(),"bump", _StorageBump);

    const [pragramPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bridge_fund_112")], program.programId);
    console.log("the pragramPDA PDA address is", pragramPDA.toBase58(),"bump", _bump);

    let from = pragramPDA; 
    let fromAta = await getAssociatedTokenAddress(mintpk, from, true);
    console.log(`from ATA addr:${fromAta.toString()}`);

    let to = new anchor.web3.PublicKey("2nhbqSk4RvZE7jsq3UZmigtmhSVSVDqrWuWN4ViStVZQ"); 
    // let to = new anchor.web3.PublicKey("HDXAyUNpESzg8EkJPojmEeNELTQS6BcVmpZ3pR5DyAfo");  // main
    let toAta = await getAssociatedTokenAddress(mintpk, to);
    console.log(`to ATA addr:${toAta.toString()}`);
    const tokenAccount = await program.provider.connection.getAccountInfo(toAta);
    if (!tokenAccount) {
      const data = new Uint8Array([
        211, 223, 118, 227, 3, 217, 119, 209, 86, 209, 126, 149, 202, 190, 149, 20,
        100, 84, 118, 187, 106, 48, 238, 210, 106, 40, 175, 86, 222, 52, 251, 3,
        240, 240, 19, 121, 149, 191, 246, 59, 226, 56, 139, 80, 209, 182, 156, 66,
        185, 97, 0, 57, 73, 239, 169, 238, 16, 190, 106, 46, 237, 182, 44, 142,
      ]);
      const payer = anchor.web3.Keypair.fromSecretKey(data);
      await createAssociatedTokenAccount(
        program.provider.connection,
        payer,
        mintpk,
        to
    );
    }

    const tx = await program.methods.withdrawFt("bridge_fund_112", new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
      myStorage:myStorage,
      pda:pragramPDA,
      to: to,
      fromAta: fromAta,
      toAta: toAta,
      mint: mintpk,
      // authority:
      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("Your transaction signature", tx);
  })
});
