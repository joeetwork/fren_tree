use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::SendRequestProps};

#[derive(Accounts)]
#[instruction(to: Pubkey)]
pub struct SendRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub from_account: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [USER, to.as_ref()],
        bump,
    )]
    pub to_account: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [REQUEST, to.as_ref(), &[to_account.requests].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<RequestAccount>() + 8,
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    #[account(
        init,
        seeds = [CONNECTION, authority.key().as_ref(), &[from_account.connections].as_ref()],
        bump,
        payer = authority,
        space = 82+36,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn send_request(ctx: Context<SendRequest>, SendRequestProps {  to }: SendRequestProps) -> Result<()> {

    let from_account = &mut ctx.accounts.from_account;

    let to_account = &mut ctx.accounts.to_account;

    let connection_account = &mut ctx.accounts.connection_account;

    let request_account = &mut ctx.accounts.request_account;

    //send request (create request pda)
    request_account.authority = to;

    request_account.from = ctx.accounts.authority.key();

    request_account.connection_number = from_account.connections;

    //increase the number of requests for receiving account
    to_account.requests = to_account.requests.checked_add(1).unwrap();

    //create connection for from
    connection_account.authority = ctx.accounts.authority.key();

    connection_account.connection = to;

    connection_account.accepted = false;

    from_account.connections = from_account.connections.checked_add(1)
    .unwrap();
    
    Ok(())
}