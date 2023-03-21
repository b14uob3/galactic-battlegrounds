use anchor_lang::prelude::*;

mod constants;
mod errors;
mod ix;
mod state;

pub use constants::*;
pub use errors::ErrorCode;
pub use ix::*;
pub use state::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod galactic_battlegrounds {
    use super::*;

    pub fn base_data_create(ctx: Context<BaseDataCreate>) -> Result<()> {
        base_data_create_inner(ctx)
    }

    pub fn base_create(ctx: Context<BaseCreate>, name: String) -> Result<()> {
        base_create_inner(ctx, name)
    }
}
