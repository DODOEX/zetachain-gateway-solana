use anchor_lang::prelude::*;

#[derive(Debug)]
#[account]
pub struct Config {
    /// owner admin
    pub owner: Pubkey,
    /// zetachain gateway
    pub gateway: Pubkey,
    /// dodo route-proxy
    pub dodo_route_proxy: Pubkey,
    /// must be initialized before using
    pub is_initialized: bool,
    /// global nonce
    pub global_nonce: u64,
    /// padding
    pub padding: [u64; 64],
}

impl Config {
    pub const LEN: usize = 8 + std::mem::size_of::<Self>();
}

#[account]
pub struct ConnectedPda {
    pub last_sender: [u8; 20],
    pub last_message: String,
    pub last_revert_sender: Pubkey,
    pub last_revert_message: String,
}

impl ConnectedPda {
    pub const LEN: usize = 8 + std::mem::size_of::<Self>();
}
