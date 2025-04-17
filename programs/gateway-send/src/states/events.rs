use anchor_lang::prelude::*;

#[event]
pub struct EddyCrossChainSwap {
    pub external_id: [u8; 32],
    pub from_token: Pubkey,
    pub to_token: Pubkey,
    pub amount: u64,
    pub output_amount: u64,
    pub wallet_address: Pubkey,
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
