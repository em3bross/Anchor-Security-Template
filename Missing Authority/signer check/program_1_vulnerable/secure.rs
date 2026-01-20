use anchor_lang::prelude::*;
declare_id!("SafeAuth11111111111111111111111111111111");
#[program]
pub mod secure_authority {
    use super::*;
    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        let data = &mut ctx.accounts.data_account;
        data.value = new_value;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub data_account: Account<'info, DataAccount>,
    pub authority: Signer<'info>,
}
#[account]
pub struct DataAccount {
    pub authority: Pubkey,
    pub value: u64,
}