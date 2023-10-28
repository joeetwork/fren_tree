use anchor_lang::prelude::*;

pub mod states;

use crate::states::*;
declare_id!("HU7c5VwLom5ShB66EDWLa8vQPHBAkeTJ9yCtJYbuGS6E");

#[program]
pub mod fren_tree {

    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, _name: String, _twitter: String, _role: String) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.name = _name;
        user_profile.twitter = _twitter;
        user_profile.role = _role;
        user_profile.upgrade = false;
        
        Ok(())
    }

    pub fn upgrade_user(ctx: Context<ChangeUserState>) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.upgrade = true;

        let current_time = Clock::get()?.unix_timestamp;

        user_profile.upgrade_time = current_time;
        
        Ok(())
    }

    pub fn check_upgrade(ctx: Context<ChangeUserState>) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        // let thirty_days_in_seconds: u64 = 30 * 24 * 60 * 60;

        // if user_profile.upgrade_time + thirty_days_in_seconds < now{
        //     user_profile.upgrade = false;
        // }
        
        Ok(())
    }

    pub fn change_role(ctx: Context<ChangeUserState>, _role: String) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.role = _role;
        
        Ok(())
    }

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"USER", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct ChangeUserState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"USER", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

}