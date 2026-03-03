use anchor_lang::prelude::*;

declare_id!("BKupsGxqcwDbNMtRvcYVpdzxpNnh4kruVHQDx2xyJLzt");

#[program]
pub mod calc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_account.count=1;
        Ok(())
    }
    pub fn Double(ctx: Context<Double>)->Result<()>{
        ctx.accounts.new_account.count=ctx.accounts.new_account.count.saturating_mul(2);
        Ok(())
    }

    pub fn Halve(ctx: Context<Double>)->Result<()>{
        ctx.accounts.new_account.count=ctx.accounts.new_account.count.saturating_div(2);
        Ok(())
    }

    pub fn Add(ctx: Context<Double>, amt:u32)->Result<()>{
        ctx.accounts.new_account.count=ctx.accounts.new_account.count.saturating_add(amt);
        Ok(())
    }

    pub fn Sub(ctx: Context<Double>, amt:u32)->Result<()>{
        ctx.accounts.new_account.count=ctx.accounts.new_account.count.saturating_sub(amt);
        Ok(())
    }
}

#[account]
pub struct NewAccount{
    count:u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=signer,space=8+4)]
    pub new_account:Account<'info,NewAccount>,
    pub system_program:Program<'info,System>,
    #[account(mut)]
    pub signer:Signer<'info>
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub new_account:Account<'info,NewAccount>,
    pub signer:Signer<'info>
}

#[derive(Accounts)]
pub struct Halve<'info> {
    #[account(mut)]
    pub new_account:Account<'info,NewAccount>,
    pub signer:Signer<'info>
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub new_account:Account<'info,NewAccount>,
    pub signer:Signer<'info>
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub new_account:Account<'info,NewAccount>,
    pub signer:Signer<'info>
}