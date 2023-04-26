use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum OnchainMetadataError {
    /// The account passed in was already initialized.
    #[error("The account has already been initialized")]
    AlreadyInitialized,

    /// The account passed isn't initialized.
    #[error("The account has not yet been initialized")]
    NotInitialized,

    #[error("The key for the JSON metadata account is invalid.")]
    MetadataDerivedKeyInvalid,

    #[error("The system program account is invalid.")]
    InvalidSystemProgram,

    #[error("The JSON data is invalid.")]
    InvalidJson,

    #[error("Borsh failed to serialize this account.")]
    BorshSerializeError,

    #[error("The payer does not have authority to perform this action.")]
    InvalidAuthority,
}

impl PrintProgramError for OnchainMetadataError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<OnchainMetadataError> for ProgramError {
    fn from(e: OnchainMetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for OnchainMetadataError {
    fn type_of() -> &'static str {
        "Error Thingy"
    }
}
