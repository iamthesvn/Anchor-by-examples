use anchor_lang::prelude::*;

declare_id!("Fvw7StQKaCmbx9ngxa3FWRbWM7dRH9Y8UJXtPxG2MGjN");

#[program]
pub mod hello_world {
    use super::*;
    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
