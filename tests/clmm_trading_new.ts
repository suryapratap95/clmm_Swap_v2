import * as anchor from '@project-serum/anchor';
import idl from "output.json";


describe('CLMM Trading New - swap_v2 Test', () => {
  // const provider = anchor.AnchorProvider.local();
  // anchor.setProvider(provider);
  // const program = anchor.workspace.ClmmTradingNew as anchor.Program;

  // Define program ID and client
  const programId = new anchor.web3.PublicKey("E39ZYh2CjA6ht8nNe5tRUKEWvBQMin8wB9Zi3iyrU8nG");
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = new anchor.Program(idl, programId, provider);

  // Hardcoded public keys (replace these with actual values for your test setup)
  const raydiumProgramId = new anchor.web3.PublicKey("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");

  const ammConfigAddress = new anchor.web3.PublicKey("CQYbhr6amxUER4p5SC44C63R4qw4NFc9Z4Db9vF4tZwG");
  const poolStateAddress = new anchor.web3.PublicKey("EMqgGrGCRn4A4sLqBPAq4wS7yCa8PatNPXQn7XBxBfm5");
  const observationStateAddress = new anchor.web3.PublicKey("8nxYw9df3sYDLoFuXcaZSiNTsUVnEMDFaeQxXtULxFFm");

  const inputVaultMintAddress = new anchor.web3.PublicKey("2qhv3WvZkB4sLc6k31Vt1o2E9YnnKSAdEwodZAv4FJnX");
  const outputVaultMintAddress = new anchor.web3.PublicKey("2qhv3WvZkB4sLc6k31Vt1o2E9YnnKSAdEwodZAv4FJnX");

  const memoProgramAddress = new anchor.web3.PublicKey("INSERT_MEMO_PROGRAM_ADDRESS_HERE");

  const userInputTokenAccount = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112");
  const userOutputTokenAccount = new anchor.web3.PublicKey("4SgPnbBaYHjPu7zH6iUD243u1zESPpNsNJV4uE6297TP");

  const poolInputVault = new anchor.web3.PublicKey("E4jPsJ12FcMofJemWKeEgaoQmUcNo35KHR4PjC6Tc1Lv");
  const poolOutputVault = new anchor.web3.PublicKey("2fqpL5t9VHCju4MWthzz7gp4dGmFMn118kfQThwzD9ST");

  const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
  const tokenProgram2022 = new anchor.web3.PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
  const memoProgram = new anchor.web3.PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

  // Hardcoded private key for the user
  const userSecretKey = Uint8Array.from([153, 218, 59, 120, 157, 190, 93, 0, 143, 197, 255, 0, 126, 2, 107, 43, 48, 237, 161, 185, 31, 172, 195, 176, 179, 137, 1, 184, 27, 174, 227, 62, 27, 85, 211, 246, 143, 57, 206, 93, 160, 75, 208, 73, 51, 38, 82, 167, 148, 41, 170, 233, 39, 78, 190, 224, 90, 78, 118, 71, 129, 82, 177, 116]);

  let user: anchor.web3.Keypair;

  before(async () => {
    // Load user keypair from the hardcoded private key
    user = anchor.web3.Keypair.fromSecretKey(userSecretKey);

    console.log('Using user with public key:', user.publicKey.toString());
  });

  it('Performs a token swap', async () => {
    // Define swap parameters
    const params = {
      amount: new anchor.BN(1000000000), // Input token amount
      otherAmountThreshold: new anchor.BN(50), // Minimum acceptable output tokens
      sqrtPriceLimitX64: new anchor.BN(0), // No price limit for this test
      isBaseInput: true, // Use base input for the swap
    };

    // Perform the swap transaction
    const tx = await program.methods.swapV2(params).accounts({
      payer: user.publicKey,
      ammConfig: ammConfigAddress,
      poolState: poolStateAddress,
      inputTokenAccount: userInputTokenAccount,
      outputTokenAccount: userOutputTokenAccount,
      inputVault: poolInputVault,
      outputVault: poolOutputVault,
      observationState: observationStateAddress,
      tokenProgram: tokenProgram,
      tokenProgram2022: tokenProgram2022,
      memoProgram: memoProgram,
      inputVaultMint: inputVaultMintAddress,
      outputVaultMint: outputVaultMintAddress,
    }).signers([user]).rpc();

    console.log('Swap transaction signature:', tx);
  });
});
