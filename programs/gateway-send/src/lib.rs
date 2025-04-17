mod errors;
mod instructions;
mod states;
mod utils;

use anchor_lang::prelude::*;

declare_id!("sNFcER7pcD5i6kMQXxd6ZdqxpF5hhYQv76VheZFzEi2");

#[program]
pub mod gateway_send {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gateway: Pubkey,
        dodo_route_proxy: Pubkey,
    ) -> Result<()> {
        instructions::initialize::initialize(ctx, gateway, dodo_route_proxy)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        gateway: Pubkey,
        dodo_route_proxy: Pubkey,
    ) -> Result<()> {
        instructions::config::initialize_config(ctx, gateway, dodo_route_proxy)
    }

    pub fn update_gateway(ctx: Context<UpdateGateway>, gateway: Pubkey) -> Result<()> {
        instructions::config::update_gateway(ctx, gateway)
    }

    pub fn update_dodo_route_proxy(
        ctx: Context<UpdateDodoRouteProxy>,
        dodo_route_proxy: Pubkey,
    ) -> Result<()> {
        instructions::config::update_dodo_route_proxy(ctx, dodo_route_proxy)
    }

    pub fn update_owner(ctx: Context<UpdateOwner>, new_owner: Pubkey) -> Result<()> {
        instructions::config::update_owner(ctx, new_owner)
    }

    pub fn close_config(ctx: Context<CloseConfig>) -> Result<()> {
        instructions::config::close_config(ctx)
    }

    pub fn deposit_and_call(
        ctx: Context<DepositAndCall>,
        amount: u64,
        target_contract: Pubkey,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_and_call::deposit_and_call(ctx, amount, target_contract, payload)
    }

    pub fn deposit_swap_and_call(
        ctx: Context<DepositSwapAndCall>,
        amount: u64,
        swap_data: Vec<u8>,
        target_contract: Pubkey,
        asset: Pubkey,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_swap_and_call::deposit_swap_and_call(
            ctx,
            amount,
            swap_data,
            target_contract,
            asset,
            payload,
        )
    }

    pub fn on_call(
        ctx: Context<OnCall>,
        external_id: [u8; 32],
        evm_wallet_address: [u8; 20],
        amount: u64,
        swap_data: Vec<u8>,
    ) -> Result<()> {
        instructions::on_call::on_call(ctx, external_id, evm_wallet_address, amount, swap_data)
    }
}
