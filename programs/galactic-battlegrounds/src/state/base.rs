use crate::*;

#[account]
pub struct Base {
    pub id: u32,
    pub name: String,
    pub owner: Pubkey,
    pub cc_level: u8,
    // todo: add alliance
}

const DISCRIMINATOR_SIZE: usize = 8;
const PUBKEY_SIZE: usize = 32;
const U8_SIZE: usize = 1;
const U32_SIZE: usize = 4;
const STRING_LENGTH_SIZE: usize = 4; // stores the string size
const MAX_NAME_SIZE: usize = 8 * 4; // max 8 chars

impl Base {
    pub fn new(id: u32, name: String, owner: Pubkey) -> Self {
        Self {
            id,
            name,
            owner,
            cc_level: 1,
        }
    }

    pub fn increment_level(&mut self) {
        self.cc_level += 1;
    }

    pub fn size() -> usize {
        U32_SIZE +
        DISCRIMINATOR_SIZE +
        STRING_LENGTH_SIZE + MAX_NAME_SIZE +
        PUBKEY_SIZE +
        U8_SIZE
    }
}
