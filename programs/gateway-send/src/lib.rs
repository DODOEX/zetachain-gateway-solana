mod errors;
mod instructions;
pub mod states;
mod utils;

use anchor_lang::prelude::*;

declare_id!("CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK");

pub const CONFIG_SEED: &[u8] = b"config";
pub const AUTHORITY_SEED: &[u8] = b"authority";
// zetachain use this
pub const CONNECTED_SEED: &[u8] = b"connected";

#[program]
pub mod gateway_send {
    pub use super::instructions::*;
    use super::*;

    pub fn create_config(
        ctx: Context<CreateConfig>,
        gateway: Pubkey,
        dodo_route_proxy: Pubkey,
    ) -> Result<()> {
        instructions::create_config(ctx, gateway, dodo_route_proxy)
    }

    pub fn update_gateway(ctx: Context<UpdateGateway>, gateway: Pubkey) -> Result<()> {
        instructions::update_gateway(ctx, gateway)
    }

    pub fn update_dodo_route_proxy(
        ctx: Context<UpdateDodoRouteProxy>,
        dodo_route_proxy: Pubkey,
    ) -> Result<()> {
        instructions::update_dodo_route_proxy(ctx, dodo_route_proxy)
    }

    pub fn update_owner(ctx: Context<UpdateOwner>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }

    pub fn deposit_sol_and_call(
        ctx: Context<DepositSolAndCall>,
        amount: u64,
        target_contract: [u8; 20],
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_sol_and_call(ctx, amount, target_contract, payload)
    }

    pub fn deposit_and_call(
        ctx: Context<DepositAndCall>,
        amount: u64,
        target_contract: Pubkey,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_and_call(ctx, amount, target_contract, payload)
    }

    pub fn deposit_swap_and_call(
        ctx: Context<DepositSwapAndCall>,
        amount: u64,
        swap_data: Vec<u8>,
        target_contract: Pubkey,
        asset: Pubkey,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_swap_and_call(ctx, amount, swap_data, target_contract, asset, payload)
    }

    pub fn on_call(
        ctx: Context<OnCall>,
        external_id: [u8; 32],
        evm_wallet_address: [u8; 20],
        amount: u64,
        swap_data: Vec<u8>,
    ) -> Result<()> {
        instructions::on_call(ctx, external_id, evm_wallet_address, amount, swap_data)
    }
}
