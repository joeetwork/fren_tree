use anchor_lang::prelude::*;

pub mod states;
pub mod constant;

use crate::{states::*, constant::*};
declare_id!("HU7c5VwLom5ShB66EDWLa8vQPHBAkeTJ9yCtJYbuGS6E");

#[program]
pub mod fren_tree {

    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, _twitter: String, _role: String) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.twitter = _twitter;
        user_profile.role = _role;
        user_profile.upgrade = false;
        user_profile.connections = 0;

        Ok(())
    }

    pub fn add_username(ctx: Context<AddUniqueName>, username: String) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let unique_username = &mut ctx.accounts.unique_username;

        unique_username.authority = ctx.accounts.authority.key();

        unique_username.username = username.clone();

        user_profile.username = username.clone();

        Ok(())
    }

    pub fn add_connection(ctx: Context<AddConnection>, _connection: Pubkey) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        let connection_account = &mut ctx.accounts.connection_account;

        connection_account.authority = ctx.accounts.authority.key();

        connection_account.connection = _connection;
       
        user_profile.connections = user_profile.connections.checked_add(1)
        .unwrap();
        
        Ok(())
    }

    pub fn remove_connection(ctx: Context<RemoveConnection>, connection_id: u8) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;
       
        user_profile.connections = user_profile.connections.checked_sub(1)
        .unwrap();
        
        Ok(())
    }

    pub fn request_count_init(ctx: Context<RequestCountInit>, request: Pubkey) -> Result<()> {

        let request_count = &mut ctx.accounts.request_count;

        request_count.authority = request;

        request_count.count = 0;
        
        Ok(())
    }

    pub fn connection_requests(ctx: Context<ConnectionRequest>, request: Pubkey) -> Result<()> {

        let request_count = &mut ctx.accounts.request_count;

        let request_account = &mut ctx.accounts.request_account;

        request_account.authority = request;

        request_account.request = ctx.accounts.authority.key();
       
        request_count.count = request_count.count.checked_add(1)
        .unwrap();
        
        Ok(())
    }

    pub fn remove_requests(ctx: Context<RemoveRequest>, request_id: u8, request: Pubkey) -> Result<()> {

        let request_count = &mut ctx.accounts.request_count;
       
        request_count.count = request_count.count.checked_sub(1)
        .unwrap();
        
        Ok(())
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

    pub fn add_top_connections(ctx: Context<ChangeTopConnections>, _connection: i32, _position: u8, _role: String) -> Result<()> {

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

    pub fn remove_top_connections(ctx: Context<ChangeTopConnections>, _position: u8, _role: String) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        if user_profile.connections < _position {
            return Ok(())
        }

        let top_connections_account = &mut ctx.accounts.top_connections_account;

        let position: usize = _position as usize;

        match _role.as_str() {
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

    pub fn upgrade_user(ctx: Context<UpgradeUser>, amount: u64) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        if user_profile.upgrade == true {
            return Ok(());
        }

        let from_account = &ctx.accounts.authority;
        //set to program owners address
        let to_account = &ctx.accounts.to;

        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(from_account.key, to_account.key, amount);

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
        seeds = [USER, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

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


#[derive(Accounts)]
#[instruction()]
pub struct AddConnection<'info> {
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
        seeds = [CONNECTION, authority.key().as_ref(), &[user_profile.connections].as_ref(), ],
        bump,
        payer = authority,
        space = std::mem::size_of::<ConnectionAccount>() + 8,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(connection_id: u8)]
pub struct RemoveConnection<'info> {
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
        close = authority,
        seeds = [CONNECTION, authority.key().as_ref(), &[connection_id].as_ref()],
        bump,
        has_one = authority,
    )]
    pub connection_account: Box<Account<'info, ConnectionAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(receiver: Pubkey)]
pub struct RequestCountInit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [REQUESTCOUNT, receiver.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<RequestCount>() + 8,
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(receiver: Pubkey)]
pub struct ConnectionRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REQUESTCOUNT, receiver.as_ref()],
        bump,
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    #[account(
        init,
        seeds = [REQUEST, request_count.authority.as_ref(), &[request_count.count].as_ref(), ],
        bump,
        payer = authority,
        space = std::mem::size_of::<RequestAccount>() + 8,
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(request_id: u8, receiver: Pubkey)]
pub struct RemoveRequest<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REQUESTCOUNT, receiver.as_ref()],
        bump,
    )]
    pub request_count: Box<Account<'info, RequestCount>>,

    #[account(
        mut,
        close = authority,
        seeds = [REQUEST, request_count.authority.as_ref(), &[request_id].as_ref(), ],
        bump,
    )]
    pub request_account: Box<Account<'info, RequestAccount>>,

    pub system_program: Program<'info, System>,
}

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

#[derive(Accounts)]
#[instruction()]
pub struct ChangeTopConnections<'info> {
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

#[derive(Accounts)]
#[instruction()]
pub struct ChangeUserState<'info> {
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

}