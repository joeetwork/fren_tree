import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { FrenTree } from '../target/types/fren_tree';
import { assert } from 'chai';

describe('fren_tree', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.FrenTree as Program<FrenTree>;

    const usersWallet = anchor.web3.Keypair.generate();

    const receiverWallet = anchor.web3.Keypair.generate();

    const connection = anchor.getProvider().connection;

    const airdrop = async () => {
        const signature = await connection.requestAirdrop(
            usersWallet.publicKey,
            anchor.web3.LAMPORTS_PER_SOL
        );

        await connection.confirmTransaction(signature);
    };

    const [usersPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [new TextEncoder().encode('USER'), usersWallet.publicKey.toBuffer()],
        program.programId
    );

    const [connectionsPda, connectonBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('CONNECTION'),
                usersWallet.publicKey.toBuffer(),
                Buffer.from([0]),
            ],
            program.programId
        );

        const [requestCountPda, requestBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUESTCOUNT'),
                receiverWallet.publicKey.toBuffer(),
            ],
            program.programId
        );

        const [requestPda, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUEST'),
                receiverWallet.publicKey.toBuffer(),
                Buffer.from([0]),
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

    it('Add connection', async () => {
        const newConnection = anchor.web3.Keypair.generate();

        await program.methods
            .addConnection(newConnection.publicKey)
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                connectionAccount: connectionsPda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Remove connection', async () => {
        await program.methods
            .removeConnection(0)
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                connectionAccount: connectionsPda,
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

    it('Init request count', async () => {

        const signature = await connection.requestAirdrop(
            receiverWallet.publicKey,
            anchor.web3.LAMPORTS_PER_SOL
        );

        await connection.confirmTransaction(signature);

        await program.methods
            .requestCountInit(receiverWallet.publicKey)
            .accounts({
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestCount: requestCountPda,
            })
            .signers([usersWallet])
            .rpc();
    });

    it('Connection Requests', async () => {

        await program.methods
            .connectionRequests(receiverWallet.publicKey)
            .accounts({
                authority: usersWallet.publicKey,
                requestCount: requestCountPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
            })
            .signers([usersWallet])
            .rpc();

            console.log(requestPda);
            
    });

    it('Remove Request', async () => {
        console.log(requestPda);

        await program.methods
            .removeRequests(0, receiverWallet.publicKey)
            .accounts({
                authority: usersWallet.publicKey,
                requestCount: requestCountPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
            })
            .signers([usersWallet])
            .rpc();
    });
});
