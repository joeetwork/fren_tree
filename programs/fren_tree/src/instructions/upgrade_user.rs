use std::str::FromStr;

use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::UpgradeUserProps};

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
        seeds = [USER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

pub fn upgrade_user(ctx: Context<UpgradeUser>) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.upgrade == true {
        return Ok(());
    }

    let from_account = &ctx.accounts.authority;

    let to_account = &ctx.accounts.to;

    let owners_wallet = &Pubkey::from_str(OWNERS_WALLET).unwrap();

    if to_account.key == owners_wallet {
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(from_account.key, to_account.key, 1000000);

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

    user_profile.upgrade_time = current_time
}
    
    Ok(())
}