use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::DeclineRequestProps};

#[derive(Accounts)]
#[instruction(request_id: u8)]
pub struct DeclineRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, request_account.sender.as_ref()],
        bump,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [REQUESTCOUNT, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    #[account(
        mut,
        close = authority,
        seeds = [REQUEST, authority.key().as_ref(), &[request_id].as_ref()],
        bump,
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    #[account(
        mut,
        close = authority,
        seeds = [CONNECTION, request_account.sender.as_ref(), &[request_account.connection_number]],
        bump,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn decline_request(ctx: Context<DeclineRequest>, params: DeclineRequestProps) -> Result<()> {
        
    let request_count = &mut ctx.accounts.request_count;

    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.connections = user_profile.connections.checked_sub(1).unwrap();

    request_count.count = request_count.count.checked_sub(1).unwrap();

    Ok(())
}