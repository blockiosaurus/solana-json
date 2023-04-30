use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::{assert_derivation, assert_signer, resize_or_reallocate_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    system_program,
};

use crate::{
    error::OnchainMetadataError, instruction::AddAuthorityArgs, pda::PREFIX, state::JsonMetadata,
};

pub(crate) fn process_add_authority(
    accounts: &[AccountInfo],
    args: AddAuthorityArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let json_account = next_account_info(account_info_iter)?;
    // Check that the account isn't already initialized.
    if (json_account.owner != &crate::ID) || json_account.data_is_empty() {
        return Err(OnchainMetadataError::NotInitialized.into());
    }

    let json_metadata_account = next_account_info(account_info_iter)?;
    // Check that the account isn't already initialized.
    if (json_metadata_account.owner != &crate::ID) || json_metadata_account.data_is_empty() {
        return Err(OnchainMetadataError::NotInitialized.into());
    }
    let mut json_metadata = JsonMetadata::try_from_slice(&json_metadata_account.data.borrow())?;

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
    if bump != json_metadata.bump {
        return Err(OnchainMetadataError::MetadataDerivedKeyInvalid.into());
    }

    let payer = next_account_info(account_info_iter)?;
    // The payer and authority must sign.
    assert_signer(payer)?;
    if !json_metadata.authorities.contains(payer.key) {
        return Err(OnchainMetadataError::InvalidAuthority.into());
    }

    let system_program = next_account_info(account_info_iter)?;
    if system_program.key != &system_program::ID {
        return Err(OnchainMetadataError::InvalidSystemProgram.into());
    }

    // Add the new authority.
    json_metadata.authorities.push(args.new_authority);

    // Write the updated JSON metadata account back to the account.
    let serialized_data = json_metadata.try_to_vec()?;

    // Resize the account to fit the new authority.
    resize_or_reallocate_account_raw(
        json_metadata_account,
        payer,
        system_program,
        serialized_data.len(),
    )?;

    // Write the JSON metadata to the JSON metadata account.
    sol_memcpy(
        &mut json_metadata_account.try_borrow_mut_data()?,
        &serialized_data,
        serialized_data.len(),
    );

    Ok(())
}
