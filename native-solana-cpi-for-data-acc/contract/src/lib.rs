use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::{ProgramResult}, lamports, program::invoke, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,entrypoint };
use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize)]
struct Counter{
    count:u32
}

#[derive(BorshDeserialize,BorshSerialize)]
enum Instruction{
 Init,
 Double,
 Half   
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{

    let ins_data=Instruction::try_from_slice(instruction_data)?;
    match ins_data{
        Instruction::Init=>{
            let mut iter=accounts.iter();
            let payer=next_account_info(&mut iter)?;
            let data_acc=next_account_info(&mut iter)?;
            let sys_program=next_account_info(&mut iter)?;

            let rent=Rent::get()?;
            let lamports=rent.minimum_balance(4);
            let ix=system_instruction::create_account(
                payer.key, data_acc.key, lamports, 4,
                 program_id);
            
            invoke(&ix, 
            &[
                payer.clone(),
                data_acc.clone(),
                sys_program.clone()
            ]);
            let counter_state=Counter{count:1};
            counter_state.serialize(&mut *data_acc.data.borrow_mut());
        },
        Instruction::Double=>{
            let mut iter=accounts.iter();
            let data_acc=next_account_info(&mut iter)?;

            let mut counter_state=Counter::try_from_slice(&mut data_acc.data.borrow())?;
            counter_state.count=counter_state.count.saturating_mul(2);
            counter_state.serialize( &mut *data_acc.data.borrow_mut());
        },
        Instruction::Half=>{
            let mut iter=accounts.iter();
            let data_acc=next_account_info(&mut iter)?;

            let mut counter_state=Counter::try_from_slice(&mut data_acc.data.borrow())?;
            counter_state.count=counter_state.count.saturating_div(2);
            counter_state.serialize( &mut *data_acc.data.borrow_mut());
        }
    }

    Ok(())

}
