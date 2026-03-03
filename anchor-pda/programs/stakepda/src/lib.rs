use anchor_lang::prelude::*;

declare_id!("DfZ4wMTD5Fa7qDPVgeWor6G3CsfTy59pD9htDFa1p7CK");

#[program]
pub mod your_program {
    use super::*;
    
    pub fn create_pda_account(_ctx: Context<CreatePdaAccount>) -> Result<()> {
        msg!("PDA account created successfully");
        Ok(())
    }
}

#[account]
pub struct StakingAccount{
    pub owner:Pubkey,
    pub staked_amt:u64,
    pub bump:u8
}

#[derive(Accounts)]
pub struct CreatePdaAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 1, // discriminator + owner + staked_amount:u64 + bump
        seeds = [b"client1", payer.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, StakingAccount>,
    
    pub system_program: Program<'info, System>,
}