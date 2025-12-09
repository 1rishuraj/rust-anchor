use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::{ProgramResult}, program_error::ProgramError,entrypoint, pubkey::Pubkey};

entrypoint!(process_instruction);

#[derive(BorshDeserialize,BorshSerialize)]
struct Counter{
    count:u32
}

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let data_acc=next_account_info(&mut accounts.iter())?;
    
    let mut cnt=Counter::try_from_slice(&data_acc.data.borrow_mut())?;
    if cnt.count==0{
        cnt.count=1;
    }else{
        cnt.count*=2;
    }
    cnt.serialize(&mut *data_acc.data.borrow_mut());
    Ok(())
}