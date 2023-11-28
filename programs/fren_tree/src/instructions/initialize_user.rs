use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::*};

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TOP, authority.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TopConnectionsAccount>() + 8,
    )]
    pub top_connections_account: Box<Account<'info, TopConnectionsAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_user(ctx: Context<InitializeUser>, InitializeUserParams { twitter, role }: InitializeUserParams) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    let top_connections_account = &mut ctx.accounts.top_connections_account;

    user_profile.authority = ctx.accounts.authority.key();
    user_profile.twitter = twitter;
    user_profile.role = role;
    user_profile.upgrade = false;
    user_profile.connections = 0;
    user_profile.requests = 0;

    //init top connections
    top_connections_account.authority = ctx.accounts.authority.key();
    
    top_connections_account.artists = Vec::new();

    top_connections_account.devs = Vec::new();

    top_connections_account.degens = Vec::new();

    top_connections_account.influencers = Vec::new();

    Ok(())
}