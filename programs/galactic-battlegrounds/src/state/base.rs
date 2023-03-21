use crate::*;

#[account]
pub struct Base {
    pub id: u32,
    pub name: String,
    pub owner: Pubkey,
    pub cc_level: u8,
    // todo: add alliance
}

impl Base {
    pub fn size() -> usize {
        DISCRIMINATOR_SIZE
        + U32_SIZE // id
        + STRING_LENGTH_SIZE + (CHAR_SIZE * 8) // name (8 chars)
        + PUBKEY_SIZE // owner
        + U8_SIZE // cc_level
    }
}
