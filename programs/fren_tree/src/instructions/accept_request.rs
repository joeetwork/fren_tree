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
        has_one = authority
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    #[account(
        mut,
        seeds = [CONNECTION, request_account.sender.as_ref(), &[request_account.connection_number]],
        bump,
        has_one = authority
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    #[account(
        init,
        seeds = [CONNECTION, authority.key().as_ref(), &[user_profile.connections].as_ref()],
        bump,
        payer = authority,
        space =  82+36,
    )]
    pub new_connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn accept_request(ctx: Context<AcceptRequest>, params: AcceptRequestProps) -> Result<()> {

    let AcceptRequestProps { request_id } = params;
        
    let user_profile = &mut ctx.accounts.user_profile;

    let connection_account = &mut ctx.accounts.connection_account;

    let new_connection_account = &mut ctx.accounts.new_connection_account;
    
    let request_count = &mut ctx.accounts.request_count;

    let request_account = &mut ctx.accounts.request_account;

    connection_account.authority = request_account.sender;

    connection_account.connection_number = user_profile.connections;

    connection_account.connection.push(ctx.accounts.authority.key());

    //setting up the new account
    new_connection_account.authority = ctx.accounts.authority.key();

    new_connection_account.connection = vec![ctx.accounts.authority.key(), request_account.sender];

    new_connection_account.connection_number = request_account.connection_number;
   
    user_profile.connections = user_profile.connections.checked_add(1)
    .unwrap();

    request_count.count = request_count.count.checked_sub(1).unwrap();

    Ok(())
}