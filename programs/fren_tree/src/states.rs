use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub twitter: String,
    pub role: String,
    pub upgrade: bool,
    pub upgrade_time: i64,
    pub connections: u8
}

#[account]
#[derive(Default)]
pub struct ConnectionAccount {
    pub authority: Pubkey,
    pub connection: Pubkey
}

#[account]
#[derive(Default)]
pub struct TopConnectionsAccount {
    pub authority: Pubkey,
    pub devs: Vec<i32>,
    pub artists: Vec<i32>,
    pub influencers: Vec<i32>,
    pub degens: Vec<i32>
}