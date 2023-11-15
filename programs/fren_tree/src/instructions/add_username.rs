use anchor_lang::prelude::*;

use crate::{states::*, constant::*, utils::AddUniqueNameProps};

#[derive(Accounts)]
#[instruction(username: String)]
pub struct AddUniqueName<'info> {
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
        init,
        seeds = [USERNAME, username.as_bytes()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UniqueUsername>(),
    )]
    pub unique_username: Box<Account<'info, UniqueUsername>>,

    pub system_program: Program<'info, System>,
}

pub fn add_username(ctx: Context<AddUniqueName>, AddUniqueNameProps {  username }: AddUniqueNameProps) -> Result<()> {

    let user_profile = &mut ctx.accounts.user_profile;
    let unique_username = &mut ctx.accounts.unique_username;

    unique_username.authority = ctx.accounts.authority.key();

    unique_username.username = username.clone();

    user_profile.username = username.clone();

    Ok(())
}