use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

#[derive(Accounts)]
#[instruction()]
pub struct CheckUpgrade<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

pub fn check_upgrade(ctx: Context<CheckUpgrade>) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.upgrade == false {
        return Ok(());
    }

    let current_time = Clock::get()?.unix_timestamp;

    let time_difference = current_time - current_time;

    let seconds_in_30_days: i64 = 30 * 24 * 60 * 60;

    if time_difference >= seconds_in_30_days {
        user_profile.upgrade = false;
    }
    
    Ok(())
}