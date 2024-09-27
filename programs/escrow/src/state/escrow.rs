use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub make: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8
}

impl Space for Escrow {
    const INIT_SPACE: usize = ANCHOR_DISC + PUBKEY_L * 3 + U64_L * 2 + U8_L * 1;
}