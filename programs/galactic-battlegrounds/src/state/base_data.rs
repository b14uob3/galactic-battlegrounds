use crate::*;

#[account]
pub struct BaseData {
    pub authority: Pubkey,
    pub count: u32,
}

const DISCRIMINATOR_SIZE: usize = 8;
const PUBKEY_SIZE: usize = 32;
const U32_SIZE: usize = 4;

impl BaseData {
    pub fn new(authority: Pubkey) -> Self {
        Self { 
            authority, 
            count: 1,
        }
    }

    pub fn size() -> usize {
        DISCRIMINATOR_SIZE +
        PUBKEY_SIZE +
        U32_SIZE
    }
}
