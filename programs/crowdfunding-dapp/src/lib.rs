use anchor_lang::prelude::*;

declare_id!("EYH93EyjromEmPgJfp6918j7uMBMc1YKb41qVMbS5zst");

#[program]
pub mod crowdfunding_dapp {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, Description: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
