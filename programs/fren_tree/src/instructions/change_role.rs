use anchor_lang::prelude::*;

use crate::{states::*, constant::*};

#[derive(Accounts)]
#[instruction()]
pub struct CheckRole<'info> {
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

pub fn change_role(ctx: Context<CheckRole>, _role: String) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.role = _role;
    
    Ok(())
}