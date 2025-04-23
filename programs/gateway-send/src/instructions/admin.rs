use crate::states::config::Config;
use crate::states::events::{DodoRouteProxyUpdated, GatewayUpdated, OwnerUpdated};
use crate::CONFIG_SEED;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(
        init,
        payer = owner,
        space = Config::LEN,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGateway<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateDodoRouteProxy<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateOwner<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,

    pub owner: Signer<'info>,
}

pub fn create_config(
    ctx: Context<CreateConfig>,
    gateway: Pubkey,
    dodo_route_proxy: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = ctx.accounts.owner.key();
    config.gateway = gateway;
    config.dodo_route_proxy = dodo_route_proxy;
    config.is_initialized = true;
    config.global_nonce = 0;
    Ok(())
}

pub fn update_gateway(ctx: Context<UpdateGateway>, new_gateway: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.gateway = new_gateway;

    emit!(GatewayUpdated {
        gateway: new_gateway
    });

    Ok(())
}

pub fn update_dodo_route_proxy(
    ctx: Context<UpdateDodoRouteProxy>,
    new_dodo_route_proxy: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.dodo_route_proxy = new_dodo_route_proxy;

    emit!(DodoRouteProxyUpdated {
        dodo_route_proxy: new_dodo_route_proxy
    });

    Ok(())
}

pub fn update_owner(ctx: Context<UpdateOwner>, new_owner: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = new_owner;

    emit!(OwnerUpdated { owner: new_owner });

    Ok(())
}
