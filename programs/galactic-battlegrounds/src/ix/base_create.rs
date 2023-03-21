use crate::*;

#[derive(Accounts)]
pub struct BaseCreate<'info> {
    #[account(
        init,
        seeds = [b"base", counter.count.to_le_bytes().as_ref()],
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



impl<'info> BaseCreate<'_> {
    pub fn process(&mut self, name: String) -> Result<()> {
        let Self {
            base,
            counter,
            payer,
            ..
        } = self;
        base.set_inner(Base::new(counter.count, name, payer.key()));

        Ok(())
    }
}
