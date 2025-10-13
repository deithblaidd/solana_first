use solana_program::{
    account_info::AccountInfo, 
    address_lookup_table::instruction, 
    entrypoint,
    entrypoint::ProgramResult, 
    instruction::{AccountMeta, Instruction}, 
    msg, 
    program::invoke, 
    pubkey::Pubkey
};

use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    _programm_id : &Pubkey,
    _accounts : &[AccountInfo],
    data : &[u8],
) -> ProgramResult {
    msg!("Hitted caller!");

    let called_id = Pubkey::new_from_array(data[0..32].try_into().expect("Incorrect slice length"));

    let instruction = Instruction{
        program_id : called_id,
        accounts : vec![],
        data : vec![],
    };

    invoke(&instruction, &[])?;
    msg!("Caller succesfully invoked called");

    Ok(())
}