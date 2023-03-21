use crate::*;

#[derive(Accounts)]
pub struct BaseDataInit<'info> {
    #[account(
        init,
        seeds = [b"base_data"],
        bump,
        payer = payer,
        space = BaseData::size(),
    )]
    pub base_data: Account<'info, BaseData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn base_data_init_inner(ctx: Context<BaseDataInit>) -> Result<()> {
    let base_data = &mut ctx.accounts.base_data;
    let payer = &ctx.accounts.payer;

    require!(base_data.count == 0, ErrorCode::AlreadyInitialized);

    base_data.count = 1;
    base_data.authority = payer.key();

    Ok(())
}
