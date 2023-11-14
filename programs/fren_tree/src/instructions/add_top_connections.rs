use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

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

pub fn add_top_connections(ctx: Context<AddTopConnections>, _connection: i32, _position: u8, _role: String) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.connections < _position || user_profile.connections < _connection as u8{
        return Ok(())
    }

    let top_connections_account = &mut ctx.accounts.top_connections_account;

    top_connections_account.authority = ctx.accounts.authority.key();

    let position: usize = _position as usize;

    match _role.as_str() {
        ARTIST => {
            top_connections_account.artists.insert(position, _connection);
        }
        DEGEN => {
            top_connections_account.degens.insert(position, _connection);
        }
        DEVELOPER => {
            top_connections_account.devs.insert(position, _connection);
        }
        INFLUENCER => {
            top_connections_account.influencers.insert(position, _connection);
        }
        _ => {
            todo!();
        }
    }
    
    Ok(())
}