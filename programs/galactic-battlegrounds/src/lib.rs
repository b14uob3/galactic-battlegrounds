use anchor_lang::prelude::*;

mod state;
mod ix;

pub use state::*;
pub use ix::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod galactic_battlegrounds {
    use super::*;

    pub fn base_create(ctx: Context<BaseCreate>, name: String) -> Result<()> {
        ctx.accounts.process(name)
    }

    pub fn base_data_init(ctx: Context<BaseDataInit>) -> Result<()> {
        ctx.accounts.process()
    }
}
