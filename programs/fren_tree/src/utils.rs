use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct GrantInputParams {
    pub cliff_seconds: u64,
    pub duration_seconds: u64,
    pub seconds_per_slice: u64,
    pub start_unix: u64,
    pub grant_token_amount: u64,
}

pub struct GrantStateParams {
    pub revoked: bool,
    pub already_issued_token_amount: u64,
}