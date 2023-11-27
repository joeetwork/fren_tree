use anchor_lang::prelude::*;

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

use utils::*;

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

    pub fn initialize_user(ctx: Context<InitializeUser>, params: InitializeUserParams) -> Result<()> {
        instructions::initialize_user::initialize_user(ctx, params)
    }

    pub fn add_username(ctx: Context<AddUniqueName>, params: AddUniqueNameProps) -> Result<()> {
        instructions::add_username::add_username(ctx, params)
    }

    pub fn change_role(ctx: Context<CheckRole>, params: CheckRoleProps) -> Result<()> {
        instructions::change_role::change_role(ctx, params)
    }

    pub fn upgrade_user(ctx: Context<UpgradeUser>) -> Result<()> {
        instructions::upgrade_user::upgrade_user(ctx)
    }

    pub fn check_upgrade(ctx: Context<CheckUpgrade>) -> Result<()> {
        instructions::check_upgrade::check_upgrade(ctx)
    }

    pub fn send_request(ctx: Context<SendRequest>, params: SendRequestProps) -> Result<()> {
        instructions::send_request::send_request(ctx, params)
    }

    pub fn accept_request(ctx: Context<AcceptRequest>, params: AcceptRequestProps) -> Result<()> {
        instructions::accept_request::accept_request(ctx, params)
    }

    pub fn decline_request(ctx: Context<DeclineRequest>, params: DeclineRequestProps) -> Result<()> {
        instructions::decline_request::decline_request(ctx, params)
    }

    pub fn remove_connection(ctx: Context<RemoveConnection>, params: RemoveConnectionProps) -> Result<()> {
        instructions::remove_connection::remove_connection(ctx, params)
    }

    pub fn initialize_top_connections(ctx: Context<InitializeTopConnections>) -> Result<()> {
        instructions::initialize_top_connections::initialize_top_connections(ctx)
    }

    pub fn add_top_connections(ctx: Context<AddTopConnections>, params: AddTopConnectionsProps) -> Result<()> {
        instructions::add_top_connections::add_top_connections(ctx, params)
    }

    pub fn remove_top_connections(ctx: Context<RemoveTopConnections>, params: RemoveTopConnectionsProps) -> Result<()> {
        instructions::remove_top_connections::remove_top_connections(ctx, params)
    }
}