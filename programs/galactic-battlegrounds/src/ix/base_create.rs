use crate::*;

#[derive(Accounts)]
pub struct BaseCreate<'info> {
    #[account(
        init,
        seeds = [b"base", base_data.count.to_le_bytes().as_ref()],
        bump,
        payer = payer,
        space = Base::size(),
    )]
    pub base: Account<'info, Base>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"base_data"], bump)]
    pub base_data: Account<'info, BaseData>,
    pub system_program: Program<'info, System>,
}

pub fn base_create_inner(ctx: Context<BaseCreate>, name: String) -> Result<()> {
    let base = &mut ctx.accounts.base;
    let payer = &ctx.accounts.payer;
    let base_data = &mut ctx.accounts.base_data;

    base.id = base_data.count;
    base.name = name;
    base.owner = payer.key();
    base.cc_level = 1;

    base_data.count = base_data.count.checked_add(1).ok_or(ErrorCode::Overflow)?;

    Ok(())
}
