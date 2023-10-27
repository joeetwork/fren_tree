use anchor_lang::prelude::*;

declare_id!("HU7c5VwLom5ShB66EDWLa8vQPHBAkeTJ9yCtJYbuGS6E");

#[program]
pub mod fren_tree {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
