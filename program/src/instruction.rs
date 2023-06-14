use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct SetValueArgs {
    pub value: String,
}

#[repr(C)]
#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AppendValueArgs {
    pub value: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AddAuthorityArgs {
    pub new_authority: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct RemoveAuthorityArgs {
    pub authority: Pubkey,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum OnchainMetadataInstructions {
    /// Description of this instruction
    #[account(0, writable, signer, name="json_account", desc = "The account to store the metadata in.")]
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

pub fn initialize(
    _program_id: Pubkey,
    json_account: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_account, true),
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::Initialize
            .try_to_vec()
            .unwrap(),
    }
}

pub fn close(
    _program_id: Pubkey,
    json_account: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_account, false),
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::Close.try_to_vec().unwrap(),
    }
}

pub fn set_value(
    _program_id: Pubkey,
    json_account: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
    args: SetValueArgs,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_account, false),
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::SetValue(args)
            .try_to_vec()
            .unwrap(),
    }
}

pub fn append_value(
    _program_id: Pubkey,
    json_account: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
    args: AppendValueArgs,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_account, false),
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::AppendValue(args)
            .try_to_vec()
            .unwrap(),
    }
}

pub fn add_authority(
    _program_id: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
    args: AddAuthorityArgs,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::AddAuthority(args)
            .try_to_vec()
            .unwrap(),
    }
}

pub fn remove_authority(
    _program_id: Pubkey,
    json_metadata_account: Pubkey,
    payer: Pubkey,
    args: RemoveAuthorityArgs,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(json_metadata_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: OnchainMetadataInstructions::RemoveAuthority(args)
            .try_to_vec()
            .unwrap(),
    }
}
