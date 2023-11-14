use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

#[derive(Accounts)]
#[instruction()]
pub struct InitializeTopConnections<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [TOP, authority.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TopConnectionsAccount>() + 8,
    )]
    pub top_connections_account: Box<Account<'info, TopConnectionsAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_top_connections(ctx: Context<InitializeTopConnections>) -> Result<()> {

    let top_connections_account = &mut ctx.accounts.top_connections_account;

    top_connections_account.authority = ctx.accounts.authority.key();
    
    top_connections_account.artists = Vec::new();

    top_connections_account.devs = Vec::new();

    top_connections_account.degens = Vec::new();

    top_connections_account.influencers = Vec::new();

    Ok(())
}