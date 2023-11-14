use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

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
        seeds = [REQUESTCOUNT, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<RequestCount>(),
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_user(ctx: Context<InitializeUser>, twitter: String, role: String) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;
    let request_count = &mut ctx.accounts.request_count;

    user_profile.authority = ctx.accounts.authority.key();
    user_profile.twitter = twitter;
    user_profile.role = role;
    user_profile.upgrade = false;
    user_profile.connections = 0;

    request_count.authority = ctx.accounts.authority.key();
    request_count.count = 0;

    Ok(())
}