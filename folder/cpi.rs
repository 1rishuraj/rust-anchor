use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::{ProgramResult}, instruction::{AccountMeta, Instruction}, program::invoke, pubkey::Pubkey, entrypoint};
entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let mut iter=accounts.iter();
    let data_acc: &AccountInfo<'_> = next_account_info(&mut iter)?;
    let double_contractPubkey: &AccountInfo<'_> = next_account_info(&mut iter)?;
    // instruction to invoke double contract
    let ix: Instruction = Instruction {
        program_id: *double_contractPubkey.key,
        accounts: vec![AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: *data_acc.key
        }],
        data: vec![]
    };
    invoke(&ix, &[data_acc.clone()])?;
    // sending the instruction + data_Account to the Double Contract
    Ok(())
}
