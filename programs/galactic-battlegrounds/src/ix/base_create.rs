use crate::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct BaseCreate<'info> {
    #[account(
        init,
        seeds = [b"base", counter.count.to_le_bytes()],
        bump,
        payer = payer,
        space = Base::size(),
    )]
    pub base: Account<'info, Base>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u32,
}

impl<'info> BaseCreate<'_> {
    pub fn process(&mut self, name: String) -> Result<()> {
        let Self { base, counter, payer, ... } = self;
        base.set_inner(Base::new(counter.count, name, payer.key()));
        counter.count += 1;

        Ok(())
    }
}
