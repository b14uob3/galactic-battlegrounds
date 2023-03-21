use anchor_lang::prelude::*;

mod state;
mod ix;
mod errors;
mod constants.rs;

pub use state::*;
pub use ix::*;
pub use errors::ErrorCode;
pub use constants::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod galactic_battlegrounds {
    use super::*;

    pub fn base_create(ctx: Context<BaseCreate>, name: String) -> Result<()> {
        base_create_inner(ctx, name)
    }

    pub fn base_data_init(ctx: Context<BaseDataInit>) -> Result<()> {
        base_data_init_inner(ctx)
    }
}
