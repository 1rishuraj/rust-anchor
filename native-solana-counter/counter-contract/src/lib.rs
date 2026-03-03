use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(counter_contract);

#[derive(BorshSerialize, BorshDeserialize)]
enum Action {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Default)]
struct Counter {
    count: u32,
}

pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    let acc = next_account_info(&mut accounts.iter())?;
    let todo = Action::try_from_slice(instruction_data)?;

   
    let mut counter_data = if acc.data_len() == 0 {
        msg!("Initializing counter to 0");
        Counter::default() // count = 0
    } else {
        Counter::try_from_slice(&acc.data.borrow())?
    };

    match todo {
        Action::Increment(x) => {
            msg!("Incrementing count by {}", x);
            counter_data.count +=x; 
        }
        Action::Decrement(x) => {
            msg!("Decrementing count by {}", x);
            counter_data.count -=x;
        }
    }

    counter_data
        .serialize(&mut *acc.data.borrow_mut());

    msg!("Contract succeeded, new count = {}", counter_data.count);
    Ok(())
}
