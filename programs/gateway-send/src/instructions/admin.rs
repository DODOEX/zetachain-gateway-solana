use crate::{
    states::{
        config::Config,
        events::{DodoRouteProxyUpdated, GatewayUpdated, OwnerUpdated},
    },
    AUTHORITY_SEED, CONFIG_SEED,
};
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = Config::LEN,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGateway<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct UpdateDodoRouteProxy<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct UpdateGasLimit<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct UpdateOwner<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct CloseConfig<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
        has_one = owner,
        close = owner
    )]
    pub config: Account<'info, Config>,
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
    config.gas_limit = 20000000;
    config.is_initialized = true;
    config.global_nonce = 0;
    let (_, authority_bump) = Pubkey::find_program_address(&[AUTHORITY_SEED], ctx.program_id);
    config.authority_bump = authority_bump;
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

pub fn update_gas_limit(ctx: Context<UpdateGasLimit>, new_gas_limit: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.gas_limit = new_gas_limit;
    Ok(())
}

pub fn update_owner(ctx: Context<UpdateOwner>, new_owner: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = new_owner;

    emit!(OwnerUpdated { owner: new_owner });

    Ok(())
}

pub fn close_config(_ctx: Context<CloseConfig>) -> Result<()> {
    // 账户将被自动关闭，租金将返还给所有者
    Ok(())
}
