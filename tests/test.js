import { Keypair } from '@solana/web3.js';

// Generate a new keypair
const keypair = Keypair.generate();

// Convert the keys to strings
const publicKey = keypair.publicKey.toBase58();
const privateKey = Buffer.from(keypair.secretKey).toString('base64');

// Print the public and private keys
console.log('Public Key:', publicKey);
console.log('Private Key:', privateKey);
