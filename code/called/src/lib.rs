use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[cfg(not(feature = "disable-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "disable-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _programm_id : &Pubkey,
    _accounts : &[AccountInfo],
    _data : &[u8],
) -> ProgramResult {
    msg!("Hitted called!");
    Ok(())
}