use borsh::BorshSerialize;
use mpl_utils::{assert_derivation, assert_signer, create_or_allocate_account_raw};
use num_traits::ToPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_memory::sol_memcpy,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

use crate::{error::OnchainMetadataError, pda::PREFIX, state::JsonMetadata};

pub(crate) fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let json_account = next_account_info(account_info_iter)?;
    // Check that the account isn't already initialized.
    if (json_account.owner != &system_program::ID) || !json_account.data_is_empty() {
        return Err(OnchainMetadataError::AlreadyInitialized.into());
    }

    let json_metadata_account = next_account_info(account_info_iter)?;
    // Check that the account isn't already initialized.
    if (json_metadata_account.owner != &system_program::ID)
        || !json_metadata_account.data_is_empty()
    {
        return Err(OnchainMetadataError::AlreadyInitialized.into());
    }
    // Verify that the derived address is correct for the JSON metadata account.
    let bump = assert_derivation(
        &crate::ID,
        json_metadata_account,
        &[
            PREFIX.as_bytes(),
            crate::ID.as_ref(),
            json_account.key.as_ref(),
        ],
        OnchainMetadataError::MetadataDerivedKeyInvalid,
    )?;

    let payer = next_account_info(account_info_iter)?;
    // The payer and authority must sign.
    assert_signer(payer)?;

    let system_program = next_account_info(account_info_iter)?;
    if system_program.key != &system_program::ID {
        return Err(OnchainMetadataError::InvalidSystemProgram.into());
    }

    // Initialize the JSON data with a null value.
    let json_data = serde_json::Value::Null;
    let serialized_data = match serde_json::to_vec(&json_data) {
        Ok(data) => data,
        Err(_) => return Err(OnchainMetadataError::InvalidJson.into()),
    };

    // Initialize the JSON metadata account.
    solana_program::msg!("Creating JSON account");
    let rent = Rent::get()?;
    let rent_amount = rent.minimum_balance(serialized_data.len());
    invoke(
        &system_instruction::create_account(
            payer.key,
            json_account.key,
            rent_amount,
            serialized_data.len().to_u64().unwrap_or(0),
            &crate::ID,
        ),
        &[payer.clone(), json_account.clone(), system_program.clone()],
    )?;

    // Initialize the JSON metadata.
    let json_metadata = JsonMetadata {
        bump,
        mutable: true,
        authorities: vec![*payer.key],
    };

    let serialized_metadata = &json_metadata.try_to_vec()?;

    // Initialize the JSON metadata account.
    solana_program::msg!("Creating JSON Metadata account");
    create_or_allocate_account_raw(
        crate::ID,
        json_metadata_account,
        system_program,
        payer,
        serialized_metadata.len(),
        &[
            PREFIX.as_bytes(),
            crate::ID.as_ref(),
            json_account.key.as_ref(),
            &[bump],
        ],
    )?;

    // Write the JSON metadata to the JSON metadata account.
    sol_memcpy(
        &mut json_metadata_account.try_borrow_mut_data()?,
        serialized_metadata,
        serialized_metadata.len(),
    );

    Ok(())
}
