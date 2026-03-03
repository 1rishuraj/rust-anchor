use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer,Transfer};

declare_id!("5YYcZJj5EU3vLayvbg6feAvvbcEkGJehnVb759R1arZd");

//module
#[program]
pub mod transfercpi{
    use super::*;

    pub fn soltransfer(ctx:Context<Soltransfer>,amt:u64)->Result<()>{
        
        let from_acc=ctx.accounts.signer.to_account_info();
        let to_acc=ctx.accounts.recipient.to_account_info();
        let prog_id=ctx.accounts.system_prog.to_account_info();
        let cpicontext=CpiContext::new(
            prog_id, 
            Transfer{
                from:from_acc,
                to:to_acc
            }
        );
        transfer(cpicontext, amt)?;
        Ok(())
    }
}

//fxn structs
#[derive(Accounts)]
pub struct Soltransfer<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub recipient:SystemAccount<'info>,
    pub system_prog:Program<'info,System>
}