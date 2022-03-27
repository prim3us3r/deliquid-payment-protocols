use anchor_lang::prelude::*;
use solana_program::account_info::AccountInfo;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod order {
    use super::*;

    pub fn create_order(
        ctx: Context<CreateOrder>,
        name: String,
        description: String,
        quantity: i8,
    ) -> ProgramResult {
        let order = &mut ctx.accounts.order;
        let payer: &Signer = &ctx.accounts.payer;
        let recipient: &AccountInfo = &ctx.accounts.recipient;
        let clock: Clock = Clock::get().unwrap();

        order.payer = *payer.key;
        order.recipient = *recipient.key;
        order.name = name.to_string();
        order.description = description.to_string();
        order.quantity = quantity;
        order.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = payer, space = Order::LEN)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Order {
    pub payer: Pubkey,       // 32 bytes
    pub recipient: Pubkey,   // 32 bytes
    pub name: String,        // 32 bytes
    pub description: String, // 32 bytes
    pub quantity: i8,        // 1 byte
    pub timestamp: i64,      // 8 bytes
}

impl Order {
    const LEN: usize = 32 + 32 + 32 + 32 + 1 + 8;
}
