import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { FrenTree } from '../target/types/fren_tree';
import { assert } from 'chai';

describe('fren_tree', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.FrenTree as Program<FrenTree>;

    const usersWallet = anchor.web3.Keypair.generate();

    const connection = anchor.getProvider().connection;

    const airdrop = async () => {
        const signature = await connection.requestAirdrop(
            usersWallet.publicKey,
            anchor.web3.LAMPORTS_PER_SOL
        );

        await connection.confirmTransaction(signature);
    };

    const [usersPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
          new TextEncoder().encode('USER'),
          usersWallet.publicKey.toBuffer(),
      ],
      program.programId
  );

    it('Is initialized!', async () => {
        await airdrop();

        await program.methods
            .initializeUser('', '', '')
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([usersWallet])
            .rpc();

        const users = await program.account.userProfile.fetch(usersPda);
        console.log('Your transaction signature', users);
    });

    it('Upgrade User', async () => {

      const data = new anchor.BN(1000000);

      const testReciever = new anchor.web3.Keypair()

      await program.methods
      .upgradeUser(data)
      .accounts({
          authority: usersWallet.publicKey,
          userProfile: usersPda,
          systemProgram: anchor.web3.SystemProgram.programId,
          to: testReciever.publicKey
      })
      .signers([usersWallet])
      .rpc();

      const newAccountBalance = await program.provider.connection.getBalance(
        testReciever.publicKey
      );
      
      assert.strictEqual(
        newAccountBalance,
        data.toNumber(),
        "The new account should have the transferred lamports"
      );

      const users = await program.account.userProfile.fetch(usersPda);
      console.log('Your transaction signature', users);
    })

    it('Check Upgrade', async () => {

      await program.methods
      .checkUpgrade()
      .accounts({
          authority: usersWallet.publicKey,
          userProfile: usersPda,
          systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([usersWallet])
      .rpc();

      const users = await program.account.userProfile.fetch(usersPda);
      console.log('Your transaction signature', users);
    })

    it('Change Role', async () => {
      await program.methods
      .changeRole("new role")
      .accounts({
          authority: usersWallet.publicKey,
          userProfile: usersPda,
          systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([usersWallet])
      .rpc();

      const users = await program.account.userProfile.fetch(usersPda);
      console.log('Your transaction signature', users);
    })

    it('Add connection', async () => {
      await program.methods
      .changeRole("new role")
      .accounts({
          authority: usersWallet.publicKey,
          userProfile: usersPda,
          systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([usersWallet])
      .rpc();

      const users = await program.account.userProfile.fetch(usersPda);
      console.log('Your transaction signature', users);
    })


});
