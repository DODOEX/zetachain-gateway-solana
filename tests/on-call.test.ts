import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GatewaySend } from "../target/types/gateway_send";
import { PublicKey, Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createTransferInstruction } from "@solana/spl-token";
import { expect } from "chai";

describe("on_call", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.GatewaySend as Program<GatewaySend>;

    // Test accounts
    let admin: Keypair;
    let user: Keypair;
    let gateway: Keypair;
    let configPda: PublicKey;
    let authorityPda: PublicKey;
    let gatewayPda: PublicKey;

    // SPL Token accounts
    let tokenMint: PublicKey;
    let configTokenAccount: PublicKey;
    let userTokenAccount: PublicKey;

    before(async () => {
        // Airdrop SOL to admin and user
        const connection = anchor.getProvider().connection;
        admin = Keypair.generate();
        user = Keypair.generate();
        gateway = Keypair.generate();

        await connection.confirmTransaction(
            await connection.requestAirdrop(admin.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL)
        );
        await connection.confirmTransaction(
            await connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL)
        );
        await connection.confirmTransaction(
            await connection.requestAirdrop(gateway.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL)
        );

        // Find PDAs
        [configPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("connected")],
            program.programId
        );
        [authorityPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("authority")],
            program.programId
        );
        [gatewayPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("gateway")],
            program.programId
        );

        // Initialize config account
        try {
            await program.methods
                .createConfig(gatewayPda, Keypair.generate().publicKey) // Mock dodo route proxy
                .accounts({
                    owner: admin.publicKey,
                    config: configPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();
        } catch (error) {
            // Config might already exist, that's okay
            console.log("Config might already be initialized:", error.message);
        }

        // Transfer some SOL to config account for testing
        const transferTx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: admin.publicKey,
                toPubkey: configPda,
                lamports: 5 * anchor.web3.LAMPORTS_PER_SOL, // Transfer 5 SOL for testing
            })
        );

        await anchor.getProvider().sendAndConfirm(transferTx, [admin]);

        // Create SPL token mint and accounts
        tokenMint = await createMint(
            anchor.getProvider().connection,
            admin,
            admin.publicKey,
            null,
            9
        );

        // Get associated token accounts
        configTokenAccount = getAssociatedTokenAddressSync(tokenMint, configPda, true);
        userTokenAccount = getAssociatedTokenAddressSync(tokenMint, user.publicKey);

        // Create config token account if it doesn't exist
        const configTokenAccountInfo = await anchor.getProvider().connection.getAccountInfo(configTokenAccount);
        if (!configTokenAccountInfo) {
            const createAtaIx = createAssociatedTokenAccountInstruction(
                admin.publicKey,
                configTokenAccount,
                configPda,
                tokenMint
            );
            const tx = new Transaction().add(createAtaIx);
            await anchor.getProvider().sendAndConfirm(tx, [admin]);
        }

        // Create user token account if it doesn't exist
        const userTokenAccountInfo = await anchor.getProvider().connection.getAccountInfo(userTokenAccount);
        if (!userTokenAccountInfo) {
            const createAtaIx = createAssociatedTokenAccountInstruction(
                admin.publicKey,
                userTokenAccount,
                user.publicKey,
                tokenMint
            );
            const tx = new Transaction().add(createAtaIx);
            await anchor.getProvider().sendAndConfirm(tx, [admin]);
        }

        // Gateway transfers tokens to config (instead of minting)
        const gatewayTokenAccount = getAssociatedTokenAddressSync(tokenMint, gateway.publicKey);

        // Create gateway token account if it doesn't exist
        const gatewayTokenAccountInfo = await anchor.getProvider().connection.getAccountInfo(gatewayTokenAccount);
        if (!gatewayTokenAccountInfo) {
            const createAtaIx = createAssociatedTokenAccountInstruction(
                admin.publicKey,
                gatewayTokenAccount,
                gateway.publicKey,
                tokenMint
            );
            const tx = new Transaction().add(createAtaIx);
            await anchor.getProvider().sendAndConfirm(tx, [admin]);
        }

        // Mint tokens to gateway first
        await mintTo(
            anchor.getProvider().connection,
            admin,
            tokenMint,
            gatewayTokenAccount,
            admin,
            1000000000 // 1 billion tokens
        );

        // Gateway transfers tokens to config
        const gatewayTransferIx = createTransferInstruction(
            gatewayTokenAccount,
            configTokenAccount,
            gateway.publicKey,
            1000000000 // 1 billion tokens
        );
        const gatewayTransferTx = new Transaction().add(gatewayTransferIx);
        await anchor.getProvider().sendAndConfirm(gatewayTransferTx, [gateway]);
    });

    describe("SOL Transfer", () => {
        it("should transfer SOL successfully", async () => {
            const amount = new anchor.BN(1000000); // 0.001 SOL
            const externalId = Buffer.alloc(32, 1); // Mock external ID
            const outputAmount = new anchor.BN(1000000);
            const receiver = Buffer.from(user.publicKey.toString()); // UTF-8 string receiver
            const swapData = Buffer.alloc(0);

            // Encode data according to program's expected format
            const receiverLenBuf = Buffer.alloc(2);
            receiverLenBuf.writeUInt16BE(receiver.length, 0);
            const swapDataLenBuf = Buffer.alloc(2);
            swapDataLenBuf.writeUInt16BE(swapData.length, 0);
            const data = Buffer.concat([
                externalId, // 32 bytes
                Buffer.alloc(24, 0), // padding for u256 (32 bytes total)
                outputAmount.toArrayLike(Buffer, 'be', 8), // 8 bytes for u64
                receiverLenBuf, // receiver length (2 bytes, big-endian)
                swapDataLenBuf, // swap data length (2 bytes, big-endian)
                receiver, // receiver bytes
                swapData // swap data bytes
            ]);

            // console.log("Data length:", data.length);
            // console.log("Receiver length:", receiver.length);
            // console.log("Swap data length:", swapData.length);
            // console.log("Data hex:", data.toString('hex'));

            const initialBalance = await anchor.getProvider().connection.getBalance(user.publicKey);

            // Create a single transaction with both instructions
            const transaction = new anchor.web3.Transaction();

            // First instruction: Transfer SOL to config account
            transaction.add(
                anchor.web3.SystemProgram.transfer({
                    fromPubkey: gateway.publicKey,
                    toPubkey: configPda,
                    lamports: amount.toNumber(),
                })
            );

            // Second instruction: Call onCall to transfer from config to user
            const onCallIx = await program.methods
                .onCall(amount, Array.from(user.publicKey.toBuffer().slice(0, 20)), data)
                .accounts({
                    config: configPda,
                    gatewayPda: gatewayPda,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .remainingAccounts([
                    { pubkey: user.publicKey, isSigner: false, isWritable: true }, // receiver
                ])
                .instruction();

            transaction.add(onCallIx);

            // Send the transaction
            await anchor.getProvider().sendAndConfirm(transaction, [gateway]);

            const finalBalance = await anchor.getProvider().connection.getBalance(user.publicKey);
            expect(finalBalance).to.equal(initialBalance + amount.toNumber());
        });

        it("should fail with insufficient balance", async () => {
            const amount = new anchor.BN(1000000000000); // Very large amount
            const externalId = Buffer.alloc(32, 1);
            const outputAmount = new anchor.BN(1000000000000);
            const receiver = Buffer.from(user.publicKey.toString()); // UTF-8 string receiver
            const swapData = Buffer.alloc(0);

            // Create a single transaction with both instructions
            const transaction = new anchor.web3.Transaction();

            // First instruction: Transfer SOL to config account (but not enough)
            transaction.add(
                anchor.web3.SystemProgram.transfer({
                    fromPubkey: gateway.publicKey,
                    toPubkey: configPda,
                    lamports: 1000000, // Only transfer 0.001 SOL, not enough for the large amount
                })
            );

            // Encode data with proper big-endian length fields
            const receiverLenBuf = Buffer.alloc(2);
            receiverLenBuf.writeUInt16BE(receiver.length, 0);
            const swapDataLenBuf = Buffer.alloc(2);
            swapDataLenBuf.writeUInt16BE(swapData.length, 0);

            const data = Buffer.concat([
                externalId,
                Buffer.alloc(24, 0), // padding for u256 (32 bytes total)
                outputAmount.toArrayLike(Buffer, 'be', 8),
                receiverLenBuf, // receiver length (2 bytes, big-endian)
                swapDataLenBuf, // swap data length (2 bytes, big-endian)
                receiver,
                swapData
            ]);

            // Second instruction: Call onCall to transfer from config to user
            const onCallIx = await program.methods
                .onCall(amount, Array.from(user.publicKey.toBuffer().slice(0, 20)), data)
                .accounts({
                    config: configPda,
                    gatewayPda: gatewayPda,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .remainingAccounts([
                    { pubkey: user.publicKey, isSigner: false, isWritable: true },
                ])
                .instruction();

            transaction.add(onCallIx);

            try {
                await anchor.getProvider().sendAndConfirm(transaction, [gateway]);
                expect.fail("Should have thrown insufficient balance error");
            } catch (error) {
                expect(error.toString()).to.include("InsufficientBalance");
            }
        });
    });

    describe("Data Decoding", () => {
        it("should decode data correctly", async () => {
            const externalId = Buffer.alloc(32, 1);
            const outputAmount = new anchor.BN(1000);
            const receiver = Buffer.from(user.publicKey.toString()); // UTF-8 string receiver
            const swapData = Buffer.from("test swap data");

            // Create a single transaction with both instructions
            const transaction = new anchor.web3.Transaction();

            // First instruction: Transfer SOL to config account
            transaction.add(
                anchor.web3.SystemProgram.transfer({
                    fromPubkey: gateway.publicKey,
                    toPubkey: configPda,
                    lamports: 1000,
                })
            );

            // Encode data with proper big-endian length fields
            const receiverLenBuf = Buffer.alloc(2);
            receiverLenBuf.writeUInt16BE(receiver.length, 0);
            const swapDataLenBuf = Buffer.alloc(2);
            swapDataLenBuf.writeUInt16BE(swapData.length, 0);

            const data = Buffer.concat([
                externalId,
                Buffer.alloc(24, 0), // padding for u256 (32 bytes total)
                outputAmount.toArrayLike(Buffer, 'be', 8),
                receiverLenBuf, // receiver length (2 bytes, big-endian)
                swapDataLenBuf, // swap data length (2 bytes, big-endian)
                receiver,
                swapData
            ]);

            // Second instruction: Call onCall to transfer from config to user
            const onCallIx = await program.methods
                .onCall(new anchor.BN(1000), Array.from(user.publicKey.toBuffer().slice(0, 20)), data)
                .accounts({
                    config: configPda,
                    gatewayPda: gatewayPda,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .remainingAccounts([
                    { pubkey: user.publicKey, isSigner: false, isWritable: true },
                ])
                .instruction();

            transaction.add(onCallIx);

            try {
                await anchor.getProvider().sendAndConfirm(transaction, [gateway]);
                // Test passed if no error thrown
                expect(true).to.be.true;
            } catch (error) {
                console.error("Data decoding test failed:", error);
                throw error;
            }
        });
    });

    describe("SPL Token Transfer", () => {
        it("should transfer SPL token successfully", async () => {
            const amount = new anchor.BN(1000000); // 1 million tokens
            const externalId = Buffer.alloc(32, 1);
            const outputAmount = new anchor.BN(1000000);
            const receiver = Buffer.from(user.publicKey.toString());
            const swapData = Buffer.alloc(0);

            // Encode data with proper big-endian length fields
            const receiverLenBuf = Buffer.alloc(2);
            receiverLenBuf.writeUInt16BE(receiver.length, 0);
            const swapDataLenBuf = Buffer.alloc(2);
            swapDataLenBuf.writeUInt16BE(swapData.length, 0);

            const data = Buffer.concat([
                externalId,
                Buffer.alloc(24, 0),
                outputAmount.toArrayLike(Buffer, 'be', 8),
                receiverLenBuf,
                swapDataLenBuf,
                receiver,
                swapData
            ]);

            // Get initial balances
            const initialConfigBalance = await getAccount(anchor.getProvider().connection, configTokenAccount);
            const initialUserBalance = await getAccount(anchor.getProvider().connection, userTokenAccount);

            // Call onCall for SPL token transfer
            await program.methods
                .onCall(amount, Array.from(user.publicKey.toBuffer().slice(0, 20)), data)
                .accounts({
                    config: configPda,
                    gatewayPda: gatewayPda,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .remainingAccounts([
                    { pubkey: user.publicKey, isSigner: false, isWritable: true }, // user_wallet
                    { pubkey: configTokenAccount, isSigner: false, isWritable: true }, // program_token_account (from)
                    { pubkey: userTokenAccount, isSigner: false, isWritable: true }, // user_token_account (to)
                    { pubkey: tokenMint, isSigner: false, isWritable: false }, // token_mint
                ])
                .rpc();

            // Check final balances
            const finalConfigBalance = await getAccount(anchor.getProvider().connection, configTokenAccount);
            const finalUserBalance = await getAccount(anchor.getProvider().connection, userTokenAccount);

            expect(finalConfigBalance.amount).to.equal(initialConfigBalance.amount - BigInt(amount.toNumber()));
            expect(finalUserBalance.amount).to.equal(initialUserBalance.amount + BigInt(amount.toNumber()));
        });

        it("should fail with insufficient SPL token balance", async () => {
            const amount = new anchor.BN(2000000000); // 2 billion tokens (more than available)
            const externalId = Buffer.alloc(32, 1);
            const outputAmount = new anchor.BN(2000000000);
            const receiver = Buffer.from(user.publicKey.toString());
            const swapData = Buffer.alloc(0);

            // Encode data with proper big-endian length fields
            const receiverLenBuf = Buffer.alloc(2);
            receiverLenBuf.writeUInt16BE(receiver.length, 0);
            const swapDataLenBuf = Buffer.alloc(2);
            swapDataLenBuf.writeUInt16BE(swapData.length, 0);

            const data = Buffer.concat([
                externalId,
                Buffer.alloc(24, 0),
                outputAmount.toArrayLike(Buffer, 'be', 8),
                receiverLenBuf,
                swapDataLenBuf,
                receiver,
                swapData
            ]);

            try {
                await program.methods
                    .onCall(amount, Array.from(user.publicKey.toBuffer().slice(0, 20)), data)
                    .accounts({
                        config: configPda,
                        gatewayPda: gatewayPda,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                    })
                    .remainingAccounts([
                        { pubkey: user.publicKey, isSigner: false, isWritable: true }, // user_wallet
                        { pubkey: configTokenAccount, isSigner: false, isWritable: true }, // program_token_account (from)
                        { pubkey: userTokenAccount, isSigner: false, isWritable: true }, // user_token_account (to)
                        { pubkey: tokenMint, isSigner: false, isWritable: false }, // token_mint
                    ])
                    .rpc();

                expect.fail("Should have thrown insufficient balance error");
            } catch (error) {
                expect(error.toString()).to.include("InsufficientBalance");
            }
        });
    });
}); 