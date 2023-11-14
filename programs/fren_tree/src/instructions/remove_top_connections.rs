use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::RemoveTopConnectionsProps};

#[derive(Accounts)]
#[instruction()]
pub struct RemoveTopConnections<'info> {
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

pub fn remove_top_connections(ctx: Context<RemoveTopConnections>, params: RemoveTopConnectionsProps) -> Result<()> {

    let RemoveTopConnectionsProps {   position, role } = params;

    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.connections < position {
        return Ok(())
    }

    let top_connections_account = &mut ctx.accounts.top_connections_account;

    let position: usize = position as usize;

    match role.as_str() {
        ARTIST => {
            top_connections_account.artists.remove(position);
        }
        DEGEN => {
            top_connections_account.degens.remove(position);
        }
        DEVELOPER => {
            top_connections_account.devs.remove(position);
        }
        INFLUENCER => {
            top_connections_account.influencers.remove(position);
        }
        _ => todo!(),
    }

    Ok(())
}