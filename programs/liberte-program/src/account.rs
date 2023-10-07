use anchor_lang::prelude::*;

use crate::constant::*;

pub trait MaxSize {
    fn get_max_size() -> usize;
}

#[account]
pub struct Settings {
    pub node_count: u64,
}

impl MaxSize for Settings {
    fn get_max_size() -> usize {
        return 8;
    }
}

#[account]
pub struct Node {
    pub active: bool,
    pub ip_addr: String,
    pub authority: Pubkey,
}

impl MaxSize for Node {
    fn get_max_size() -> usize {
        return 1 + IP_ADDR_SIZE + 32;
    }
}
