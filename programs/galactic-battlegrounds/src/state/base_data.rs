use crate::*;

#[account]
pub struct BaseData {
    pub count: u32,
}

const DISCRIMINATOR_SIZE: usize = 8;
const U32_SIZE: usize = 4;

impl BaseData {
    pub fn new() -> Self {
        Self { count: 1 }
    }

    pub fn size() -> usize {
        DISCRIMINATOR_SIZE +
        U32_SIZE
    }
}
