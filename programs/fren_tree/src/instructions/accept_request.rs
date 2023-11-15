use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::AcceptRequestProps};

#[derive(Accounts)]
#[instruction(request_id: u8)]
pub struct AcceptRequest<'info> {
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
        close = authority,
        seeds = [REQUEST, authority.key().as_ref(), &[request_id].as_ref()],
        bump,
        has_one = authority
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    #[account(
        mut,
        seeds = [CONNECTION, request_account.from.as_ref(), &[request_account.connection_number]],
        bump,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    #[account(
        init,
        seeds = [CONNECTION, authority.key().as_ref(), &[to_account.connections].as_ref()],
        bump,
        payer = authority,
        space =  82+36,
    )]
    pub new_connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn accept_request(ctx: Context<AcceptRequest>, params: AcceptRequestProps) -> Result<()> {

    let to_account = &mut ctx.accounts.to_account;

    let connection_account = &mut ctx.accounts.connection_account;

    let new_connection_account = &mut ctx.accounts.new_connection_account;

    let request_account = &mut ctx.accounts.request_account;

    //edit from accounts connection to be accepted
    connection_account.accepted = true;


    //create connection account for receiver
    new_connection_account.authority = ctx.accounts.authority.key();

    new_connection_account.connection = request_account.from;

    new_connection_account.connection_number = request_account.connection_number;

    new_connection_account.accepted = true;
   
    to_account.connections = to_account.connections.checked_add(1)
    .unwrap();

    //remove request
    to_account.requests = to_account.requests.checked_sub(1).unwrap();

    Ok(())
}