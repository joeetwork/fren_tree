use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use solana_program::system_instruction;

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

    pub fn upgrade_user(ctx: Context<UpgradeUser>, amount: u64) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        if user_profile.upgrade == true {
            return Ok(());
        }

        let from_account = &ctx.accounts.authority;
        //set to program owners address
        let to_account = &ctx.accounts.to;

        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        user_profile.upgrade = true;

        let current_time = Clock::get()?.unix_timestamp;

        user_profile.upgrade_time = current_time;
        
        Ok(())
    }

    pub fn check_upgrade(ctx: Context<ChangeUserState>) -> Result<()> {

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

#[derive(Accounts)]
#[instruction()]
pub struct UpgradeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    //for testing purposes
     /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,

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