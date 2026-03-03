use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::create_account,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, //this contract's pubkey
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    //as payer is seed for pda so (needed to verify)
    let payer_account = next_account_info(iter)?;
    let payer_pubkey = payer_account.key;
    
    /*pda is passed To verify PDA correctness
    Because a malicious user might pass any writable account, not the PDA you derived, and then your program would create/fund the wrong account.*/
    let pda_account = next_account_info(iter)?;
    
    //the program Id to be invoked must be passed because we CPI into it
    let system_program = next_account_info(iter)?;

    let (pda, bump) = Pubkey::find_program_address(
        &[b"client1", payer_pubkey.as_ref()],
        &program_id, //this programid must be of this contract which is creating the pda , just like ATAP for ATAs
    );
    /*
    - Now to create a PDA for another user , payer_pubkey  while accessing this contract
    - b"client1" (namespace) remains constant as "client1" suggests "user PDA"
    eg.
    User	Seeds	                    Result
    Alice	["client1", alice_pubkey]	PDA₁
    Bob	    ["client1", bob_pubkey]	    PDA₂
    Carol	["client1", carol_pubkey]	PDA₃
    */

    //to check PDA is correct or not
    assert_eq!(*pda_account.key, pda);


    let ix = create_account(
     &payer_account.key,
       &pda,
        1000000000,
           4, 
        //space 4 for future storage of any u32(4 bytes) data on the PDA, say count(u32)
        // eg. let pda_account = next_account_info(iter)?;
        //     let mut count_bytes = pda_account.data.borrow_mut();
        program_id, //this contract which we are writing is the owner of pda
    );
    let signer_seeds = &[b"client1", payer_pubkey.as_ref(), &[bump]];

    //invoke_signed expects the instruction, 
    //accounts in order that SYSTEM-PROGRAM.createaccount expects
    //signer_seeds : [&[&u8]] ie.  all the seeds must be in binary in a 2D array [[seed1_binary],[seed2_binary][bump]] 
    //Bump must be included
    invoke_signed(&ix, accounts, &[signer_seeds])?;


    Ok(())
}

