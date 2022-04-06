use anchor_lang::prelude::*;
use solana_program::account_info::AccountInfo;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deliquid_payment {
    use super::*;

    pub fn create_merchant(ctx: Context<CreateMerchant>, name: String) -> ProgramResult {
        let merchant = &mut ctx.accounts.merchant;
        let owner = &mut ctx.accounts.owner;

        if name.chars().count() > 30 {
            return Err(ErrorCode::MerchantNameTooLong.into());
        }

        merchant.owner = *owner.key;
        merchant.name = name;
        Ok(())
    }

    pub fn update_merchant(ctx: Context<UpdateMerchant>, name: String) -> ProgramResult {
        let merchant = &mut ctx.accounts.merchant;

        if name.chars().count() > 30 {
            return Err(ErrorCode::MerchantNameTooLong.into());
        }

        merchant.name = name;
        Ok(())
    }

    pub fn create_charge(ctx: Context<CreateCharge>, description: String) -> ProgramResult {
        let charge = &mut ctx.accounts.charge;
        let customer: &Signer = &ctx.accounts.customer;
        let merchant: &AccountInfo = &ctx.accounts.merchant;

        if description.chars().count() > 280 {
            return Err(ErrorCode::ChargeDescriptionTooLong.into());
        }

        charge.customer = *customer.key;
        charge.merchant = *merchant.key;
        charge.description = description.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMerchant<'info> {
    #[account(init, payer = owner, space = Merchant::LEN)]
    pub merchant: Account<'info, Merchant>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMerchant<'info> {
    #[account(mut, has_one = owner)]
    pub merchant: Account<'info, Merchant>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateCharge<'info> {
    #[account(init, payer = customer, space = Charge::LEN)]
    pub charge: Account<'info, Charge>,
    #[account(mut)]
    pub customer: Signer<'info>,
    #[account(mut)]
    pub merchant: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Charge {
    pub customer: Pubkey,    // 32 bytes
    pub merchant: Pubkey,    // 32 bytes
    pub description: String, // 280 * 4 bytes (280 chars)
}

#[account]
pub struct Merchant {
    pub owner: Pubkey, // 32 bytes
    pub name: String,  // 30 * 4 bytes (30 chars)
}

impl Charge {
    const LEN: usize = 32 + 32 + 280 * 4;
}

impl Merchant {
    const LEN: usize = 32 + 30 * 4;
}

#[error]
pub enum ErrorCode {
    #[msg("The merchant name should be 30 characters long maximum.")]
    MerchantNameTooLong,
    #[msg("The charge description should be 280 characters long maximum.")]
    ChargeDescriptionTooLong,
}
