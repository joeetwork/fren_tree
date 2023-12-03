use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::DeclineRequestProps};

#[derive(Accounts)]
#[instruction(request_id: u8)]
pub struct DeclineRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub to_account: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [USER, request_account.from.as_ref()],
        bump,
    )]
    pub from_account: Box<Account<'info, UserProfile>>,

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
        seeds = [CONNECTION, request_account.from.as_ref(), &[request_account.connection_number]],
        bump,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn decline_request(ctx: Context<DeclineRequest>, params: DeclineRequestProps) -> Result<()> {
        
    let from_account = &mut ctx.accounts.from_account;

    let to_account = &mut ctx.accounts.to_account;

    from_account.connection_count = from_account.connection_count.checked_sub(1).unwrap();

    to_account.request_count = to_account.request_count.checked_sub(1).unwrap();

    Ok(())
}