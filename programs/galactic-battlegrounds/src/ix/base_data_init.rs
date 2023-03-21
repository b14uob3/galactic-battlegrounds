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
    pub fn process(&mut self) -> Result<()> {
        let Self {
            base_data,
            payer,
            ..
        } = self;

        if base_data.count != 0 {
            return Err(ErrorCode::BaseDataAlreadyInitialized.into());
        }

        base_data.set_inner(BaseData::new());
        Ok(())
    }
}