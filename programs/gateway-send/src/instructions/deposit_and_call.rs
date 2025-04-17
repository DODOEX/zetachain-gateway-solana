use crate::errors::GatewayError;
use crate::states::config::Config;
use crate::states::events::EddyCrossChainSwap;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub user_from_token_account: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer = user, space = TokenAccount::LEN, seeds = [b"program_token", user_from_token_account.mint.as_ref()], bump)]
    pub program_from_token_account: Account<'info, TokenAccount>,

    #[account(address = config.gateway)]
    pub gateway: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_and_call(
    ctx: Context<DepositAndCall>,
    amount: u64,
    target_contract: Pubkey,
    payload: Vec<u8>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let user = &ctx.accounts.user;

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;

    // Transfer tokens from user to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_from_token_account.to_account_info(),
        to: ctx.accounts.program_from_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Prepare account metas
    let account_metas = prepare_account_metas_only_gateway(&ctx.remaining_accounts, &user)?;

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: gateway_account_metas,
        data: payload,
    };

    invoke_signed(
        &gateway_ix,
        &ctx.remaining_accounts[route_proxy_account_metas.len() + 2..],
        &[],
    )?;

    // Emit event
    emit!(EddyCrossChainSwap {
        external_id,
        from_token: ctx.accounts.user_from_token_account.mint,
        to_token: ctx.accounts.user_from_token_account.mint,
        amount,
        output_amount: amount,
        wallet_address: user.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DepositSwapAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(mut, constraint = user_from_token_account.mint != asset_mint.key())]
    pub user_from_token_account: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer = user, space = TokenAccount::LEN, seeds = [b"program_token", user_from_token_account.mint.as_ref()], bump)]
    pub program_from_token_account: Account<'info, TokenAccount>,

    #[account(mint::token_program = token_program)]
    pub asset_mint: Account<'info, Mint>,

    #[account(init_if_needed, payer = user, space = TokenAccount::LEN, seeds = [b"program_token", asset_mint.key().as_ref()], bump)]
    pub program_asset_token_account: Account<'info, TokenAccount>,

    #[account(address = config.dodo_route_proxy)]
    pub dodo_route_proxy: AccountInfo<'info>,

    #[account(address = config.gateway)]
    pub gateway: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_swap_and_call(
    ctx: Context<DepositSwapAndCall>,
    amount: u64,
    swap_data: Vec<u8>,
    target_contract: Pubkey,
    asset: Pubkey,
    payload: Vec<u8>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let user = &ctx.accounts.user;

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;

    // Transfer tokens from user to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_from_token_account.to_account_info(),
        to: ctx.accounts.program_from_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Prepare account metas
    let (route_proxy_account_metas, gateway_account_metas) =
        prepare_account_metas(&ctx.remaining_accounts, &user, &ctx.accounts.gateway.key())?;

    // Call DODO Route Proxy for token swap
    let swap_ix = Instruction {
        program_id: ctx.accounts.dodo_route_proxy.key(),
        accounts: route_proxy_account_metas,
        data: swap_data,
    };
    invoke_signed(
        &swap_ix,
        &ctx.remaining_accounts[1..route_proxy_account_metas.len() + 1],
        &[],
    )
    .map_err(|_| GatewayError::RouteProxyCallFailed)?;
    let result_account = &ctx.remaining_accounts[0];
    let result_data = result_account.try_borrow_data()?;
    let result = u64::from_le_bytes(result_data[..8].try_into().unwrap());

    // Decode output amount
    let output_amount = u64::from_le_bytes(swap_result.try_into().unwrap());

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: gateway_account_metas,
        data: payload,
    };

    invoke_signed(
        &gateway_ix,
        &ctx.remaining_accounts[route_proxy_account_metas.len() + 2..],
        &[],
    )?;

    // Emit event
    emit!(EddyCrossChainSwap {
        external_id,
        from_token: ctx.accounts.user_from_token_account.mint,
        to_token: asset,
        amount,
        output_amount,
        wallet_address: user.key(),
    });

    Ok(())
}

pub fn calc_external_id(
    program_id: &Pubkey,
    sender: &Pubkey,
    global_nonce: u64,
) -> Result<[u8; 32]> {
    let timestamp = Clock::get()?.unix_timestamp as u64;
    let hash_from_multiple = anchor_lang::solana_program::hash::hashv(&[
        &program_id.to_bytes(),
        &sender.to_bytes(),
        &global_nonce.to_le_bytes(),
        &timestamp.to_le_bytes(),
    ]);
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash_from_multiple.to_bytes());
    Ok(result)
}
