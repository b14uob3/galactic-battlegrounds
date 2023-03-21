use crate::*;

#[account]
pub struct BaseData {
    pub authority: Pubkey,
    pub count: u32,
}

impl BaseData {
    pub fn size() -> usize {
        DISCRIMINATOR_SIZE
        + PUBKEY_SIZE // authority
        + U32_SIZE // count
    }
}
