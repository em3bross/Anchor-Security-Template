use anchor_lang::prelude::*;
declare_id!("SafePDA1111111111111111111111111111111");
#[program]
pub mod secure_pda {
    use super::*;
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance += amount;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}
#[account]
pub struct Vault {
    pub balance: u64,
}