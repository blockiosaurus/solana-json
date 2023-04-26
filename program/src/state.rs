use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct JsonMetadata {
    pub bump: u8,
    pub mutable: bool,
    pub authorities: Vec<Pubkey>,
}
