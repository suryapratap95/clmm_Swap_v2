import * as anchor from '@project-serum/anchor';
import { SystemProgram } from '@solana/web3.js';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { assert } from 'chai';

// Define program ID and client
const programId = new anchor.web3.PublicKey("E39ZYh2CjA6ht8nNe5tRUKEWvBQMin8wB9Zi3iyrU8nG");
const raydiumProgramId = new anchor.web3.PublicKey("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");
const provider = anchor.Provider.local();
anchor.setProvider(provider);

const program = new anchor.Program(idl, programId, provider);

describe('CLMM Trading New', () => {
  let user: anchor.web3.Keypair;
  let poolState: anchor.web3.Keypair;
  let tokenMint0: anchor.web3.Keypair;
  let tokenMint1: anchor.web3.Keypair;
  let userToken0Account: anchor.web3.PublicKey;
  let userToken1Account: anchor.web3.PublicKey;
  let poolToken0Vault: anchor.web3.PublicKey;
  let poolToken1Vault: anchor.web3.PublicKey;

  before(async () => {
    user = anchor.web3.Keypair.generate();
    tokenMint0 = anchor.web3.Keypair.generate();
    tokenMint1 = anchor.web3.Keypair.generate();

    // Create mint for token 0 and token 1
    await createMint(tokenMint0);
    await createMint(tokenMint1);

    // Create associated token accounts for the user
    userToken0Account = await createAssociatedTokenAccount(user, tokenMint0.publicKey);
    userToken1Account = await createAssociatedTokenAccount(user, tokenMint1.publicKey);

    // Create pool token vaults
    poolToken0Vault = await createAssociatedTokenAccount(user, tokenMint0.publicKey);
    poolToken1Vault = await createAssociatedTokenAccount(user, tokenMint1.publicKey);

    // Initialize pool state
    poolState = anchor.web3.Keypair.generate();
  });

  // Helper function to create a mint
  async function createMint(mint: anchor.web3.Keypair) {
    await program.provider.connection.sendTransaction(
      new anchor.web3.Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: provider.wallet.publicKey,
          newAccountPubkey: mint.publicKey,
          lamports: await provider.provider.connection.getMinimumBalanceForRentExemption(
            anchor.web3.MintLayout.span
          ),
          space: anchor.web3.MintLayout.span,
          programId: TOKEN_PROGRAM_ID,
        }),
        Token.createInitMintInstruction(TOKEN_PROGRAM_ID, mint.publicKey, 0, provider.wallet.publicKey, null)
      ),
      [mint],
      { skipPreflight: false, preflightCommitment: 'processed' }
    );
  }

  // Helper function to create an associated token account
  async function createAssociatedTokenAccount(owner: anchor.web3.Keypair, mint: anchor.web3.PublicKey) {
    const associatedToken = await Token.getAssociatedTokenAddress(
      Token.associatedProgramId,
      TOKEN_PROGRAM_ID,
      mint,
      owner.publicKey
    );
    const transaction = new anchor.web3.Transaction().add(
      Token.createAssociatedTokenAccountInstruction(
        Token.associatedProgramId,
        TOKEN_PROGRAM_ID,
        mint,
        associatedToken,
        owner.publicKey,
        owner.publicKey
      )
    );
    await program.provider.send(transaction, [owner]);
    return associatedToken;
  }

  it('Initializes the pool', async () => {
    // Initialize pool
    const tx = await program.rpc.initializePool(new anchor.BN(1000), new anchor.BN(1_000_000_000), {
      accounts: {
        authority: user.publicKey,
        poolState: poolState.publicKey,
        tokenMint0: tokenMint0.publicKey,
        tokenMint1: tokenMint1.publicKey,
        tokenVault0: poolToken0Vault,
        tokenVault1: poolToken1Vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user, poolState],
    });

    console.log('Pool initialized with tx: ', tx);
  });

  it('Creates liquidity', async () => {
    // Create liquidity params
    const params = {
      liquidityDelta: new anchor.BN(1000),
      tickLowerIndex: -100,
      tickUpperIndex: 100,
      amount0Max: new anchor.BN(1000),
      amount1Max: new anchor.BN(1000),
    };

    // Create liquidity transaction
    const tx = await program.rpc.createLiquidity(params, {
      accounts: {
        user: user.publicKey,
        poolState: poolState.publicKey,
        userToken0Account: userToken0Account,
        userToken1Account: userToken1Account,
        poolToken0Vault: poolToken0Vault,
        poolToken1Vault: poolToken1Vault,
        raydiumProgram: raydiumProgramId,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [user],
    });

    console.log('Liquidity created with tx: ', tx);
  });

  it('Performs swap', async () => {
    // Swap parameters
    const params = {
      amountIn: new anchor.BN(100),
      minAmountOut: new anchor.BN(90),
      sqrtPriceLimitX64: new anchor.BN(0),
      isBaseInput: true,
      swapDirection: true,
      otherAmountThreshold: new anchor.BN(0),
    };

    // Perform swap
    const tx = await program.rpc.swapV2(params, {
      accounts: {
        user: user.publicKey,
        poolState: poolState.publicKey,
        ammConfig: poolState.publicKey,  // Assuming some default configuration
        userSourceToken: userToken0Account,
        userDestinationToken: userToken1Account,
        poolSourceVault: poolToken0Vault,
        poolDestinationVault: poolToken1Vault,
        observationState: poolState.publicKey,
        tickArray: poolState.publicKey,
        raydiumProgram: raydiumProgramId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user],
    });

    console.log('Swap performed with tx: ', tx);
  });
});

