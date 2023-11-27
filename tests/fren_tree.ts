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

    const [connectionPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            new TextEncoder().encode('CONNECTION'),
            usersWallet.publicKey.toBuffer(),
            Buffer.from([0]),
        ],
        program.programId
    );

    const [newConnectionPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            new TextEncoder().encode('CONNECTION'),
            randomWallet.publicKey.toBuffer(),
            Buffer.from([0]),
        ],
        program.programId
    );

    it('Is initialized!', async () => {
        await airdrop();

        const params = { twitter: '', role: '' };

        await program.methods
            .initializeUser(params)
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
            .addUsername({ username: 'user1' })
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
        const ownersWallet = new anchor.web3.PublicKey(
            '2TgV6sWTaHt8Tdca1qHVNwTtFEvRDXsKE3yqzoPL3Mvs'
        );

        await program.methods
            .upgradeUser()
            .accounts({
                authority: usersWallet.publicKey,
                userProfile: usersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                to: ownersWallet,
            })
            .signers([usersWallet])
            .rpc();

        const newAccountBalance = await program.provider.connection.getBalance(
            ownersWallet
        );

        assert.strictEqual(
            newAccountBalance,
            new anchor.BN(1000000).toNumber(),
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
            .changeRole({ role: 'new role' })
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
            .addTopConnections({ connection: 0, position: 0, role: 'Degen' })
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
            .removeTopConnections({ position: 0, role: 'Degen' })
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

        const params = { twitter: '', role: '' };

        await program.methods
            .initializeUser(params)
            .accounts({
                authority: randomWallet.publicKey,
                userProfile: randomUsersPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([randomWallet])
            .rpc();
    });

    it('Send Request', async () => {
        const recieverCount = await program.account.userProfile.fetch(
            randomUsersPda
        );

        const [requestPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUEST'),
                randomWallet.publicKey.toBuffer(),
                Buffer.from([recieverCount.requests]),
            ],
            program.programId
        );

        await program.methods
            .sendRequest({ to: randomWallet.publicKey })
            .accounts({
                fromAccount: usersPda,
                toAccount: randomUsersPda,
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
                connectionAccount: connectionPda,
            })
            .signers([usersWallet])
            .rpc();

        const test = await program.account.requestAccount.fetch(requestPda);

        console.log(test);
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
            .acceptRequest({ requestId: 0 })
            .accounts({
                toAccount: randomUsersPda,
                authority: randomWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                requestAccount: requestPda,
                connectionAccount: connectionPda,
                newConnectionAccount: newConnectionPda,
            })
            .signers([randomWallet])
            .rpc();

        const test = await program.account.userProfile.fetch(usersPda);

        console.log(test);
    });

    // it('Decline Request', async () => {
    //     const [requestPda] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [
    //             new TextEncoder().encode('REQUEST'),
    //             randomWallet.publicKey.toBuffer(),
    //             Buffer.from([0]),
    //         ],
    //         program.programId
    //     );

    //     await program.methods
    //         .declineRequest({requestId: 0})
    //         .accounts({
    //             fromAccount: usersPda,
    //             toAccount: randomUsersPda,
    //             authority: randomWallet.publicKey,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //             requestAccount: requestPda,
    //             connectionAccount: connectionPda,
    //         })
    //         .signers([randomWallet])
    //         .rpc();

    //     const test = await program.account.userProfile.fetch(usersPda);

    //     console.log(test);
    // });

    it('Remove Connection', async () => {
        const [requestPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                new TextEncoder().encode('REQUEST'),
                randomWallet.publicKey.toBuffer(),
                Buffer.from([0]),
            ],
            program.programId
        );

        await program.methods
            .removeConnection({ connectionId: 0 })
            .accounts({
                fromAccount: usersPda,
                toAccount: randomUsersPda,
                fromConnectionAccount: connectionPda,
                toConnectionAccount: newConnectionPda,
                authority: usersWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([usersWallet])
            .rpc();

        const test = await program.account.userProfile.fetch(usersPda);

        console.log(test);
    });
});
