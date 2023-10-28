use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub twitter: String,
    pub role: String,
    pub upgrade: bool,
    pub upgrade_time: i64
}