use anchor_lang::prelude::*;

declare_id!("EYH93EyjromEmPgJfp6918j7uMBMc1YKb41qVMbS5zst");

#[program]
pub mod crowdfunding_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
