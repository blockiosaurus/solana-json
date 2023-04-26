use solana_program::pubkey::Pubkey;

pub const PREFIX: &str = "JSON";

pub fn find_metadata_account(json_account: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PREFIX.as_bytes(), json_account.as_ref()], &crate::id())
}
