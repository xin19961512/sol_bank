
use anchor_lang::{prelude::*};
// Event
#[event]
pub struct DepositNative {
    pub from: Pubkey,
    pub to: Pubkey,
    pub value: u64,
    pub chain: String,
    pub addr: String
}

#[event]
pub struct DepositFt {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint:Pubkey,
    pub value: u64,
    pub chain: String,
    pub addr: String
}

#[event]
pub struct WithdrawNative {
    pub to: Pubkey,
    pub value: u64,
}

#[event]
pub struct WithdrawFt {
    pub to: Pubkey,
    pub mint:Pubkey,
    pub value: u64,
}

// Error
#[error_code]
pub enum BridgeError {
    #[msg("Only owner can call this function!")]
    NotOwner,
    #[msg("Deposit to pda ,no sufficient amount!")]
    DepositNE,
    #[msg("Withdraw from pda ,no sufficient amount!")]
    WithdrawNE,
    #[msg("PDA not fetch to the key")]
    WrongPDA,
    #[msg("ATA not fetch to the key")]
    WrongATA
}