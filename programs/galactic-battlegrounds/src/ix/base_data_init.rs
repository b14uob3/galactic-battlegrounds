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

impl<'info> BaseDataInit<'_> {
    #[access_control(Self::constraints(&self))]
    pub fn process(&mut self) -> Result<()> {
        let Self {
            base_data,
            payer,
            ..
        } = self;

        base_data.set_inner(BaseData::new(payer.key()));
        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        let Self {
            base_data,
            payer,
            ..
        } = self;

        require!(base_data.count == 0, ErrorCode::AlreadyInitialized);
        Ok(())
    }
}