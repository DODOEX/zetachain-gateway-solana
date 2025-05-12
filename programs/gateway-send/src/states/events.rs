use anchor_lang::prelude::*;

#[event]
pub struct EddyCrossChainRevert {
    pub external_id: [u8; 32],
    pub token: Pubkey,
    pub amount: u64,
    pub wallet_address: Pubkey,
}

#[event]
pub struct EddyCrossChainSend {
    pub external_id: [u8; 32],
    pub dst_chain_id: u32,
    pub from_token: Pubkey,
    pub to_token: Pubkey,
    pub amount: u64,
    pub output_amount: u64,
    pub wallet_address: Pubkey,
    pub payload: Vec<u8>,
}

#[event]
pub struct EddyCrossChainReceive {
    pub external_id: [u8; 32],
    pub from_token: Pubkey,
    pub to_token: Pubkey,
    pub amount: u64,
    pub output_amount: u64,
    pub wallet_address: Pubkey,
    pub payload: Vec<u8>,
}

#[event]
pub struct DodoRouteProxyUpdated {
    pub dodo_route_proxy: Pubkey,
}

#[event]
pub struct GatewayUpdated {
    pub gateway: Pubkey,
}

#[event]
pub struct OwnerUpdated {
    pub owner: Pubkey,
}
