use crate::errors::GatewayError;
use crate::states::config::Config;
use crate::states::events::EddyCrossChainSwap;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct OnCall<'info> {
    #[account(mut, constraint = gateway.key() == config.gateway)]
    pub gateway: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN, seeds = [b"program_token", from_token_account.mint.as_ref()], bump)]
    pub program_from_token_account: Account<'info, TokenAccount>,

    pub user: AccountInfo<'info>,

    #[account(mint::token_program = token_program)]
    pub to_mint: Option<Account<'info, Mint>>,

    #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN)]
    pub program_to_token_account: Option<Account<'info, TokenAccount>>,

    #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(address = config.dodo_route_proxy)]
    pub dodo_route_proxy: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn on_call(
    ctx: Context<OnCall>,
    external_id: [u8; 32],
    evm_wallet_address: [u8; 20],
    amount: u64,
    swap_data: Vec<u8>,
) -> Result<()> {
    if ctx.accounts.to_mint.is_some() {
        require!(
            ctx.accounts.program_to_token_account.is_some()
                && ctx.accounts.program_to_token_account.unwrap().mint
                    == ctx.accounts.to_mint.unwrap().key()
                && ctx.accounts.user_token_account.mint == ctx.accounts.to_mint.unwrap().key()
                && ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
            GatewayError::InvalidInstructionData
        );
    } else {
        require!(
            ctx.accounts.program_to_token_account.is_none()
                && ctx.accounts.user_token_account.mint == ctx.accounts.from_token_account.mint
                && ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
            GatewayError::InvalidInstructionData
        );
    }
    let config = &ctx.accounts.config;
    let gateway = &ctx.accounts.gateway;

    // Transfer tokens from gateway to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.program_from_token_account.to_account_info(),
        authority: gateway.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    let mut from_token_account = &ctx.accounts.program_from_token_account;
    let mut output_amount = amount;
    if ctx.accounts.to_mint.is_some() {
        // Call DODO Route Proxy for token swap
        let account_metas =
            prepare_account_metas_only_route_proxy(&ctx.remaining_accounts, &ctx.accounts.gateway)?;
        let swap_ix = Instruction {
            program_id: ctx.accounts.dodo_route_proxy.key(),
            accounts: account_metas,
            data: swap_data,
        };
        invoke_signed(&swap_ix, &ctx.remaining_accounts[1..], &[])
            .map_err(|_| GatewayError::RouteProxyCallFailed)?;

        // Get output amount from remaining accounts
        let data = ctx.remaining_accounts[0].try_borrow_data()?;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[..8]);
        output_amount = u64::from_le_bytes(bytes);
        from_token_account = ctx.accounts.program_to_token_account.as_ref().unwrap();
    }

    // Transfer tokens to User wallet
    let cpi_accounts = Transfer {
        from: from_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: gateway.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, output_amount)?;

    // Emit event
    emit!(EddyCrossChainSwap {
        external_id,
        from_token: ctx.accounts.from_token_account.mint,
        to_token: ctx
            .accounts
            .to_mint
            .unwrap_or(&ctx.accounts.from_token_account.mint),
        amount,
        output_amount,
        wallet_address: ctx.accounts.user.key(),
    });

    Ok(())
}
