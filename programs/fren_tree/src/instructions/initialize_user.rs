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
        seeds = [CONNECTION, authority.key().as_ref(), &[user_profile.connections].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<ConnectionAccount>(),
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_user(ctx: Context<InitializeUser>, InitializeUserParams { twitter, role }: InitializeUserParams) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    let connection_account = &mut ctx.accounts.connection_account;

    user_profile.authority = ctx.accounts.authority.key();
    user_profile.twitter = twitter;
    user_profile.role = role;
    user_profile.upgrade = false;
    user_profile.connections = 0;
    user_profile.requests = 0;

    //create connection for from
    connection_account.authority = ctx.accounts.authority.key();

    connection_account.accepted = false;

    user_profile.connections = 0;

    Ok(())
}