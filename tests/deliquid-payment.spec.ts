import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DeliquidPayment } from '../target/types/deliquid_payment';
import { Keypair, SystemProgram } from '@solana/web3.js';

describe('deliquid-payment', () => {
  anchor.setProvider(anchor.Provider.env());

  const connection = anchor.getProvider().connection;
  const program = anchor.workspace.DeliquidPayment as Program<DeliquidPayment>;
  const merchant = Keypair.generate();
  const merchantOwner = Keypair.generate();
  const customer = Keypair.generate();

  it('Create a merchant account!', async () => {
    const signature = await program.methods
      .createMerchant('First Merchant')
      .accounts({
        merchant: merchant.publicKey,
        owner: merchantOwner.publicKey,
      })
      .signers([merchant])
      .rpc();

    console.log(signature);
  });

  it('Update a merchant account!', async () => {
    const tx = await program.methods
      .updateMerchant('Merchant name updated')
      .accounts({
        merchant: merchant.publicKey,
        owner: merchantOwner.publicKey,
      })
      .signers([merchantOwner])
      .rpc();
    console.log(tx);
  });

  it('Create a charge by a customer', async () => {
    const charge = Keypair.generate();

    const tx = await program.methods
      .createCharge('First Charge')
      .accounts({
        charge: charge.publicKey,
        customer: customer.publicKey,
        merchant: merchantOwner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([customer])
      .rpc();
    console.log(tx);
  });
});
