use anchor_lang::prelude::*;

declare_id!("5YYcZJj5EU3vLayvbg6feAvvbcEkGJehnVb759R1arZd");
#[program]
pub mod transferContract{

    use anchor_lang::prelude::{instruction::Instruction, program::invoke};

    use super::*;
    pub fn soltranfer(ctx:Context<SolTransfer>,amt:u64)->Result<()>{
        let sender=ctx.accounts.signer.to_account_info();
        let recipient=ctx.accounts.recipient.to_account_info();
        let sys_program=ctx.accounts.program.to_account_info();

        //create accountsmeta array for instruction
        let acc_meta=vec![
            AccountMeta::new(*sender.key, true),
            AccountMeta::new(*recipient.key, false),
            ];

        //discriminator
        let ins_discriminator:u32=2;

        //ins_data to give to instruction
        let mut ins_data=Vec::with_capacity(4+8); //4 for u32 discriminator, 8 for u64 lamports
        ins_data.extend_from_slice(&ins_discriminator.to_le_bytes());//store in little endian
        ins_data.extend_from_slice(&amt.to_le_bytes());//store in little endian
        //instruction
        let ins_data=Instruction{
            program_id:sys_program.key(),//to invoke into
            accounts:acc_meta,
            data:ins_data,
        };
        //invoke
        invoke(&ins_data, &[sender,recipient,sys_program])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info>{
    #[account(mut)]
    signer:Signer<'info>,
    #[account(mut)]
    recipient:SystemAccount<'info>,
    program:Program<'info,System>
}