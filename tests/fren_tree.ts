import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { FrenTree } from '../target/types/fren_tree';
import { assert } from 'chai';

describe('fren_tree', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.FrenTree as Program<FrenTree>;

    const usersWallet = anchor.web3.Keypair.generate();

    const randomWallet = anchor.web3.Keypair.generate();

    const connection = anchor.getProvider().connection;

    const airdrop = async () => {
        const signature = await connection.requestAirdrop(
            usersWallet.publicKey,
            anchor.web3.LAMPORTS_PER_SOL
        );

        await connection.confirmTransaction(signature);
    };

    const airdrop2 = async () => {
        const signature = await connection.requestAirdrop(
            randomWallet.publicKey,
            anchor.web3.LAMPORTS_PER_SOL
        );

        await connection.confirmTransaction(signature);
    };

    const [usersPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [new TextEncoder().encode('USER'), usersWallet.publicKey.toBuffer()],
        program.programId
    );

    const [randomUsersPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [new TextEncoder().encode('USER'), randomWallet.publicKey.toBuffer()],
        program.programId
    );

    const [randomRequestCountsPda] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUESTCOUNT'),
                randomWallet.publicKey.toBuffer(),
            ],
            program.programId
        );

    const [requestCountsPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            new TextEncoder().encode('REQUESTCOUNT'),
            usersWallet.publicKey.toBuffer(),
        ],
        program.programId
    );

    const [connectionPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            new TextEncoder().encode('CONNECTION'),
            usersWallet.publicKey.toBuffer(),
        ],
        program.programId
    );

    const [newConnectionPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            new TextEncoder().encode('CONNECTION'),
            randomWallet.publicKey.toBuffer(),
        ],
        program.programId
    );

    it('Is initialized!', async () => {
        await airdrop();

        await program.methods
            .initializeUser('', '')
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                requestCount: requestCountsPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Add Username', async () => {
        await airdrop();

        const [usernamePda, bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    new TextEncoder().encode('USERNAME'),
                    new TextEncoder().encode('user1'),
                ],
                program.programId
            );

        await program.methods
            .addUsername('user1')
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                uniqueUsername: usernamePda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Upgrade User', async () => {
        const data = new anchor.BN(1000000);

        const testReciever = new anchor.web3.Keypair();

        await program.methods
            .upgradeUser(data)
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                to: testReciever.publicKey,
            })
            .signers([usersWallet])
            .rpc();

        const newAccountBalance = await program.provider.connection.getBalance(
            testReciever.publicKey
        );

        assert.strictEqual(
            newAccountBalance,
            data.toNumber(),
            'The new account should have the transferred lamports'
        );
    });

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
    });

    it('Change Role', async () => {
        await program.methods
            .changeRole('new role')
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Init top connections', async () => {
        const [topConnectionsPda, bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    new TextEncoder().encode('TOP'),
                    usersWallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        await program.methods
            .initializeTopConnections()
            .accounts({
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                topConnectionsAccount: topConnectionsPda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Add top connection', async () => {
        const [topConnectionsPda, bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    new TextEncoder().encode('TOP'),
                    usersWallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        await program.methods
            .addTopConnections(0, 0, 'Degen')
            .accounts({
                userProfile: usersPda,
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                topConnectionsAccount: topConnectionsPda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Remove top connection', async () => {
        const [topConnectionsPda, bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    new TextEncoder().encode('TOP'),
                    usersWallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        await program.methods
            .removeTopConnections(0, 'Degen')
            .accounts({
                userProfile: usersPda,
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                topConnectionsAccount: topConnectionsPda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Create Random Wallet account!', async () => {
        await airdrop2();

        await program.methods
            .initializeUser('', '')
            .accounts({
                authority: randomWallet.publicKey,
                userProfile: randomUsersPda,
                requestCount: randomRequestCountsPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([randomWallet])
            .rpc();
    });

    it('Send Request', async () => {
        const recieverCount = await program.account.requestCount.fetch(
            randomRequestCountsPda
        );

        const [requestPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUEST'),
                randomWallet.publicKey.toBuffer(),
                Buffer.from([recieverCount.count]),
            ],
            program.programId
        );

        await program.methods
            .sendRequest(randomWallet.publicKey)
            .accounts({
                userProfile: usersPda,
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
                requestCount: randomRequestCountsPda,
                connectionAccount: connectionPda,
            })
            .signers([usersWallet])
            .rpc();

        const test = await program.account.requestAccount.fetch(
            requestPda
        );

        console.log(requestPda);
    });

    it('Accept Request', async () => {

        const [requestPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUEST'),
                randomWallet.publicKey.toBuffer(),
                Buffer.from([0]),
            ],
            program.programId
        );

        await program.methods
            .acceptRequest(0)
            .accounts({
                userProfile: randomUsersPda,
                authority: randomWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
                requestCount: randomRequestCountsPda,
                connectionAccount: connectionPda,
                newConnectionAccount: newConnectionPda
            })
            .signers([randomWallet])
            .rpc();

        const test = await program.account.connectionAccount.fetch(
            newConnectionPda
        );

        console.log(test);
    });
});
