use anchor_lang::prelude::*;

mod account_data;
mod instructions;
mod utils;

pub mod states;
pub mod constant;

use instructions::initialize_user::*;
use instructions::add_username::*;
use instructions::change_role::*;
use instructions::upgrade_user::*;
use instructions::check_upgrade::*;
use instructions::send_request::*;
use instructions::accept_request::*;
use instructions::decline_request::*;
use instructions::remove_connection::*;
use instructions::initialize_top_connections::*;
use instructions::add_top_connections::*;
use instructions::remove_top_connections::*;

use utils::GrantInputParams;

declare_id!("HU7c5VwLom5ShB66EDWLa8vQPHBAkeTJ9yCtJYbuGS6E");

#[error_code]
pub enum TokenVestingError {
    #[msg("Grant input parameters invalid")]
    ParamsInvalid,
    // #[msg("Employer put a 0 token grant! call the union!")]
    // EmployerNGMI,
}

#[program]
pub mod fren_tree {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, twitter: String, role: String) -> Result<()> {
        instructions::initialize_user::initialize_user(ctx, twitter, role)
    }

    pub fn add_username(ctx: Context<AddUniqueName>, username: String) -> Result<()> {
        instructions::add_username::add_username(ctx, username)
    }

    pub fn change_role(ctx: Context<CheckRole>, _role: String) -> Result<()> {
        instructions::change_role::change_role(ctx, _role)
    }

    pub fn upgrade_user(ctx: Context<UpgradeUser>, amount: u64) -> Result<()> {
        instructions::upgrade_user::upgrade_user(ctx, amount)
    }

    pub fn check_upgrade(ctx: Context<CheckUpgrade>) -> Result<()> {
        instructions::check_upgrade::check_upgrade(ctx)
    }

    pub fn send_request(ctx: Context<SendRequest>, receiver: Pubkey) -> Result<()> {
        instructions::send_request::send_request(ctx, receiver)
    }

    pub fn accept_request(ctx: Context<AcceptRequest>, _request_id: u8) -> Result<()> {
        instructions::accept_request::accept_request(ctx, _request_id)
    }

    pub fn decline_request(ctx: Context<DeclineRequest>, _request_id: u8) -> Result<()> {
        instructions::decline_request::decline_request(ctx, _request_id)
    }

    pub fn remove_connection(ctx: Context<RemoveConnection>, _connection_id: u8) -> Result<()> {
        instructions::remove_connection::remove_connection(ctx, _connection_id)
    }

    pub fn initialize_top_connections(ctx: Context<InitializeTopConnections>) -> Result<()> {
        instructions::initialize_top_connections::initialize_top_connections(ctx)
    }

    pub fn add_top_connections(ctx: Context<AddTopConnections>, _connection: i32, _position: u8, _role: String) -> Result<()> {
        instructions::add_top_connections::add_top_connections(ctx, _connection, _position, _role)
    }

    pub fn remove_top_connections(ctx: Context<RemoveTopConnections>, _position: u8, _role: String) -> Result<()> {
        instructions::remove_top_connections::remove_top_connections(ctx, _position, _role)
    }
}