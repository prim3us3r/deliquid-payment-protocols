use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod order {
    use super::*;
    pub fn initialize(ctx: Context<CreateOrder>, name: String, description: String, quantity: i8) -> ProgramResult {
        let order = &mut ctx.accounts.order;
        let payer: &Signer = &ctx.accounts.payer;
        let clock: Clock = Clock::get().unwrap();

        order.payer = *payer.key;
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
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Order {
    pub payer: Pubkey,
    pub recipient: Pubkey,
    pub name: String,
    pub description: String,
    pub quantity: i8,
    pub timestamp: i64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const QUANTITY_LENGTH: usize = 8;
const NAME_LENGTH: usize = 32;
const DESCRIPTION_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;

impl Order {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + QUANTITY_LENGTH
        + NAME_LENGTH
        + DESCRIPTION_LENGTH
        + TIMESTAMP_LENGTH;
}
