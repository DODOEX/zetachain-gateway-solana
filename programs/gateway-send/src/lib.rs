mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK");

pub const CONFIG_SEED: &[u8] = CONNECTED_SEED;
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

    pub fn update_gas_limit(ctx: Context<UpdateGasLimit>, new_gas_limit: u64) -> Result<()> {
        instructions::update_gas_limit(ctx, new_gas_limit)
    }

    pub fn update_owner(ctx: Context<UpdateOwner>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }

    pub fn close_config(ctx: Context<CloseConfig>) -> Result<()> {
        instructions::close_config(ctx)
    }

    pub fn deposit_sol_and_call(
        ctx: Context<DepositSolAndCall>,
        target_contract: [u8; 20],
        amount: u64,
        dst_chain_id: u32,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_sol_and_call(ctx, target_contract, amount, dst_chain_id, payload)
    }

    pub fn deposit_spl_and_call<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositSplAndCall<'info>>,
        target_contract: [u8; 20],
        amount: u64,
        asset: Pubkey,
        dst_chain_id: u32,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_spl_and_call(
            ctx,
            target_contract,
            amount,
            asset,
            dst_chain_id,
            payload,
        )
    }

    pub fn deposit_spl_swap_spl_and_call<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositSplSwapSplAndCall<'info>>,
        target_contract: [u8; 20],
        amount: u64,
        swap_data: Vec<u8>,
        asset: Pubkey,
        dst_chain_id: u32,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_spl_swap_spl_and_call(
            ctx,
            target_contract,
            amount,
            swap_data,
            asset,
            dst_chain_id,
            payload,
        )
    }

    pub fn deposit_spl_swap_sol_and_call<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositSplSwapSolAndCall<'info>>,
        target_contract: [u8; 20],
        amount: u64,
        swap_data: Vec<u8>,
        asset: Pubkey,
        dst_chain_id: u32,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_spl_swap_sol_and_call(
            ctx,
            target_contract,
            amount,
            swap_data,
            asset,
            dst_chain_id,
            payload,
        )
    }

    pub fn on_call<'info>(
        ctx: Context<'_, '_, 'info, 'info, OnCall<'info>>,
        amount: u64,
        sender: [u8; 20],
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::on_call(ctx, amount, sender, data)
    }

    pub fn on_revert<'info>(
        ctx: Context<'_, '_, '_, 'info, OnRevert<'info>>,
        amount: u64,
        sender: Pubkey,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::on_revert(ctx, amount, sender, data)
    }
}
