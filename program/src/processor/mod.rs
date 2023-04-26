use crate::instruction::OnchainMetadataInstructions;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

mod add_authority;
mod append_value;
mod close;
mod initialize;
mod remove_authority;
mod set_value;

use add_authority::*;
use append_value::*;
use close::*;
use initialize::*;
use remove_authority::*;
use set_value::*;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: OnchainMetadataInstructions =
            OnchainMetadataInstructions::try_from_slice(instruction_data)?;
        match instruction {
            OnchainMetadataInstructions::Initialize => {
                msg!("Instruction: Initialize");
                process_initialize(accounts)
            }
            OnchainMetadataInstructions::Close => {
                msg!("Instruction: Close");
                process_close(accounts)
            }
            OnchainMetadataInstructions::SetValue(args) => {
                msg!("Instruction: SetValue");
                process_set_value(accounts, args)
            }
            OnchainMetadataInstructions::AppendValue(args) => {
                msg!("Instruction: AppendValue");
                process_append_value(accounts, args)
            }
            OnchainMetadataInstructions::AddAuthority(args) => {
                msg!("Instruction: AddAuthority");
                process_add_authority(accounts, args)
            }
            OnchainMetadataInstructions::RemoveAuthority(args) => {
                msg!("Instruction: RemoveAuthority");
                process_remove_authority(accounts, args)
            }
        }
    }
}
