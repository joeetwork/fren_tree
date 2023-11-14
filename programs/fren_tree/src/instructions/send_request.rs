use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

#[derive(Accounts)]
#[instruction(receiver: Pubkey)]
pub struct SendRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [REQUESTCOUNT, receiver.as_ref()],
        bump,
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    #[account(
        init,
        seeds = [REQUEST, receiver.as_ref(), &[request_count.count].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<RequestAccount>() + 8,
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    #[account(
        init,
        seeds = [CONNECTION, authority.key().as_ref(), &[user_profile.connections].as_ref()],
        bump,
        payer = authority,
        space = 82+36,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn send_request(ctx: Context<SendRequest>, receiver: Pubkey) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    let connection_account = &mut ctx.accounts.connection_account;

    let request_count = &mut ctx.accounts.request_count;

    let request_account = &mut ctx.accounts.request_account;

    connection_account.authority = receiver;

    connection_account.connection = vec![ctx.accounts.authority.key()];

    //need to have a check to see if the user has a request count pda
    request_count.count = request_count.count.checked_add(1).unwrap();

    request_account.authority = receiver;

    request_account.sender = ctx.accounts.authority.key();

    request_account.connection_number = user_profile.connections;

    user_profile.connections = user_profile.connections.checked_add(1)
    .unwrap();

//need to add a way for the receiver account to find the created request
    
    Ok(())
}