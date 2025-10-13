use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _programm_id : &Pubkey,
    _accounts : &[AccountInfo],
    _data : &[u8],
) -> ProgramResult {
    msg!("Hitted called!");
    Ok(())
}