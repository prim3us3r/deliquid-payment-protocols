import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DeliquidPayment } from '../target/types/deliquid_payment';
import { Keypair } from '@solana/web3.js';

describe('deliquid-payment', () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DeliquidPayment as Program<DeliquidPayment>;
  const merchantOwner = Keypair.generate();
  const customer = Keypair.generate();

  it('Create a merchant account!', async () => {
    const tx = await program.methods
      .createMerchant('First Merchant')
      .accounts({
        owner: merchantOwner.publicKey,
      })
      .signers([merchantOwner])
      .rpc();
    console.log(tx);
  });

  it('Update a merchant account!', async () => {
    const tx = await program.methods
      .updateMerchant('Merchant name updated')
      .accounts({
        owner: merchantOwner.publicKey,
      })
      .signers([merchantOwner])
      .rpc();
    console.log(tx);
  });

  it('Create a charge by a customer', async () => {
    const tx = await program.methods
      .createCharge('First Charge')
      .accounts({
        customer: customer.publicKey,
        merchant: merchantOwner.publicKey,
      })
      .signers([customer])
      .rpc();
    console.log(tx);
  });
});
