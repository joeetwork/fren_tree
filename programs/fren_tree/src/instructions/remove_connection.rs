use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::RemoveConnectionProps};

#[derive(Accounts)]
#[instruction(connection_id: u8)]
pub struct RemoveConnection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, from_connection_account.authority.as_ref()],
        bump,
    )]
    pub from_account: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [USER, from_connection_account.connection.as_ref()],
        bump,
    )]
    pub to_account: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [CONNECTION, authority.key().as_ref(), &[connection_id]],
        bump,
        has_one = authority
    )]
    pub from_connection_account: Box<Account<'info, ConnectionAccount>>,

    #[account(
        mut,
        close = authority,
        seeds = [CONNECTION, from_connection_account.connection.as_ref(), &[from_connection_account.connection_number].as_ref()],
        bump,
    )]
    pub to_connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn remove_connection(ctx: Context<RemoveConnection>, params: RemoveConnectionProps) -> Result<()> {

    let from_account = &mut ctx.accounts.from_account;

    let to_account = &mut ctx.accounts.to_account;

    from_account.connections = from_account.connections.checked_sub(1).unwrap();

    to_account.connections = to_account.connections.checked_sub(1).unwrap();
    
    Ok(())
}