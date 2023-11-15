use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::AddTopConnectionsProps};

#[derive(Accounts)]
#[instruction()]
pub struct AddTopConnections<'info> {
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
        seeds = [TOP, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub top_connections_account: Box<Account<'info, TopConnectionsAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn add_top_connections(ctx: Context<AddTopConnections>, AddTopConnectionsProps {  connection, position, role }: AddTopConnectionsProps) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.connections < position || user_profile.connections < connection as u8{
        return Ok(())
    }

    let top_connections_account = &mut ctx.accounts.top_connections_account;

    top_connections_account.authority = ctx.accounts.authority.key();

    let position: usize = position as usize;

    match role.as_str() {
        ARTIST => {
            top_connections_account.artists.insert(position, connection);
        }
        DEGEN => {
            top_connections_account.degens.insert(position, connection);
        }
        DEVELOPER => {
            top_connections_account.devs.insert(position, connection);
        }
        INFLUENCER => {
            top_connections_account.influencers.insert(position, connection);
        }
        _ => {
            todo!();
        }
    }
    
    Ok(())
}