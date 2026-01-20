use anchor_lang::prelude::*;
declare_id!("SafeMath1111111111111111111111111111111");
#[program]
pub mod secure_math {
    use super::*;
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.balance = account
        .balance
        .checked_sub(amount)
        .ok_or(ErrorCode::InsufficientBalance)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub account: Account<'info, BalanceAccount>,
}
#[account]
pub struct BalanceAccount {
    pub balance: u64,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}