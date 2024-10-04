use anchor_lang::prelude::*;

declare_id!("4xaExS7aH9yUh6EYACM9vf3pzSKSqtFh3unB79qfuDxF");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
