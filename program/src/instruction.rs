use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct SetValueArgs {
    pub value: String,
}

#[repr(C)]
#[repr(C)]
#[derive(PartialEq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AppendValueArgs {
    pub value: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct AddAuthorityArgs {
    pub new_authority: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct RemoveAuthorityArgs {
    pub authority: Pubkey,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum OnchainMetadataInstructions {
    /// Description of this instruction
    #[account(0, writable, name="json_account", desc = "The account to store the metadata in.")]
    #[account(1, writable, name="json_metadata_account", desc = "The account to store the json account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(3, name="system_program", desc = "System program")]
    Initialize,

    /// Description of this instruction
    #[account(0, writable, name="json_account", desc = "The account to store the metadata in.")]
    #[account(1, writable, name="json_metadata_account", desc = "The account to store the json account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(3, name="system_program", desc = "System program")]
    Close,

    /// Description of this instruction
    #[account(0, writable, name="json_account", desc = "The account to store the metadata in.")]
    #[account(1, writable, name="json_metadata_account", desc = "The account to store the json account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(3, name="system_program", desc = "System program")]
    SetValue(SetValueArgs),

    /// Description of this instruction
    #[account(0, writable, name="json_account", desc = "The account to store the metadata in.")]
    #[account(1, writable, name="json_metadata_account", desc = "The account to store the json account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(3, name="system_program", desc = "System program")]
    AppendValue(AppendValueArgs),

    #[account(0, writable, name="json_metadata_account", desc = "The account to store the metadata's metadata in.")]
    #[account(1, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(2, name="system_program", desc = "System program")]
    AddAuthority(AddAuthorityArgs),

    #[account(0, writable, name="json_metadata_account", desc = "The account to store the metadata's metadata in.")]
    #[account(1, writable, signer, name="payer", desc="The account that will pay for the transaction and rent.")]
    #[account(2, name="system_program", desc = "System program")]
    RemoveAuthority(RemoveAuthorityArgs),
}
