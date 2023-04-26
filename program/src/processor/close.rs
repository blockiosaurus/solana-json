use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, assert_signer, close_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

use crate::{error::OnchainMetadataError, pda::PREFIX, state::JsonMetadata};

pub(crate) fn process_close(accounts: &[AccountInfo]) -> ProgramResult {
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
    let json_metadata = JsonMetadata::try_from_slice(&json_metadata_account.data.borrow())?;

    // Verify that the derived address is correct for the JSON metadata account.
    let bump = assert_derivation(
        &crate::ID,
        json_metadata_account,
        &[PREFIX.as_bytes(), json_account.key.as_ref()],
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

    // Close both accounts
    close_account_raw(payer, json_account)?;
    close_account_raw(payer, json_metadata_account)?;

    Ok(())
}
