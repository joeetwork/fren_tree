use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::RemoveConnectionProps};

#[derive(Accounts)]
#[instruction(connection_id: u8)]
pub struct RemoveConnection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, from_connection_account.connection[0].as_ref()],
        bump,
    )]
    pub from: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [USER, from_connection_account.connection[1].as_ref()],
        bump,
    )]
    pub to: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [CONNECTION, authority.key().as_ref(), &[connection_id]],
        bump,
    )]
    pub from_connection_account: Box<Account<'info, ConnectionAccount>>,

    #[account(
        mut,
        close = authority,
        seeds = [CONNECTION, from_connection_account.connection[1].as_ref(), &[from_connection_account.connection_number].as_ref()],
        bump,
    )]
    pub to_connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn remove_connection(ctx: Context<RemoveConnection>, params: RemoveConnectionProps) -> Result<()> {

    let RemoveConnectionProps {  connection_id } = params;

    let from = &mut ctx.accounts.from;

    let to = &mut ctx.accounts.to;

    from.connections = from.connections.checked_sub(1).unwrap();

    to.connections = to.connections.checked_sub(1).unwrap();
    
    Ok(())
}