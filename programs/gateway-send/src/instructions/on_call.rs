use {
    crate::{
        errors::GatewayError,
        states::{config::Config, events::EddyCrossChainReceive},
        utils::prepare_account_metas_only_route_proxy,
        CONFIG_SEED,
    },
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, program::invoke_signed},
    },
    anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct OnCall<'info> {
    // #[account(mut, constraint = gateway.key() == config.gateway)]
    // pub gateway: Signer<'info>,
    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    // #[account(mut)]
    // pub from_token_account: Account<'info, TokenAccount>,

    // #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN, seeds = [b"program_token", from_token_account.mint.as_ref()], bump)]
    // pub program_from_token_account: Account<'info, TokenAccount>,

    // pub user: AccountInfo<'info>,

    // #[account(mint::token_program = token_program)]
    // pub to_mint: Option<Account<'info, Mint>>,

    // #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN)]
    // pub program_to_token_account: Option<Account<'info, TokenAccount>>,

    // #[account(init_if_needed, payer = gateway, space = TokenAccount::LEN)]
    // pub user_token_account: Account<'info, TokenAccount>,

    // #[account(address = config.dodo_route_proxy)]
    // pub dodo_route_proxy: AccountInfo<'info>,

    // pub token_program: Program<'info, Token>,
    /// CHECK: Test contract
    pub gateway_pda: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn on_call(ctx: Context<OnCall>, amount: u64, sender: [u8; 20], data: Vec<u8>) -> Result<()> {
    msg!("on_call {:?} {} {:?}", sender, amount, data);
    // let mut offset = 0;

    // // 解码主消息
    // let external_id = decode_bytes32(&data, &mut offset);
    // let evm_wallet_address = decode_address(&data, &mut offset);
    // let amount = decode_u256(&data, &mut offset);
    // let cross_chain_swap_data = decode_bytes(&data, &mut offset);

    // // 解码跨链交换数据
    // let mut swap_offset = 0;
    // let from_token = decode_address(&cross_chain_swap_data, &mut swap_offset);
    // let to_token = decode_address(&cross_chain_swap_data, &mut swap_offset);
    // let swap_data = decode_bytes(&cross_chain_swap_data, &mut swap_offset);

    // if ctx.accounts.to_mint.is_some() {
    //     require!(
    //         ctx.accounts.program_to_token_account.as_ref().is_some()
    //             && ctx.accounts.program_to_token_account.as_ref().unwrap().mint
    //                 == ctx.accounts.to_mint.as_ref().unwrap().key()
    //             && ctx.accounts.user_token_account.mint
    //                 == ctx.accounts.to_mint.as_ref().unwrap().key()
    //             && ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
    //         GatewayError::InvalidInstructionData
    //     );
    // } else {
    //     require!(
    //         ctx.accounts.program_to_token_account.is_none()
    //             && ctx.accounts.user_token_account.mint == ctx.accounts.from_token_account.mint
    //             && ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
    //         GatewayError::InvalidInstructionData
    //     );
    // }

    // // Transfer tokens from gateway to program
    // let cpi_accounts = Transfer {
    //     from: ctx.accounts.from_token_account.to_account_info(),
    //     to: ctx.accounts.program_from_token_account.to_account_info(),
    //     authority: ctx.accounts.gateway.to_account_info(),
    // };
    // let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    // token::transfer(cpi_ctx, amount)?;

    // let mut from_token_account = &ctx.accounts.program_from_token_account;
    // let mut output_amount = amount;
    // if ctx.accounts.to_mint.is_some() {
    //     // Call DODO Route Proxy for token swap
    //     let account_metas =
    //         prepare_account_metas_only_route_proxy(ctx.remaining_accounts, &ctx.accounts.gateway)?;
    //     let swap_ix = Instruction {
    //         program_id: ctx.accounts.dodo_route_proxy.key(),
    //         accounts: account_metas,
    //         data: swap_data,
    //     };
    //     invoke_signed(&swap_ix, &ctx.remaining_accounts[1..], &[])
    //         .map_err(|_| GatewayError::RouteProxyCallFailed)?;

    //     // Get output amount from remaining accounts
    //     let data = ctx.remaining_accounts[0].try_borrow_data()?;
    //     let mut bytes = [0u8; 8];
    //     bytes.copy_from_slice(&data[..8]);
    //     output_amount = u64::from_le_bytes(bytes);
    //     from_token_account = ctx.accounts.program_to_token_account.as_ref().unwrap();
    // }

    // // Transfer tokens to User wallet
    // let cpi_accounts = Transfer {
    //     from: from_token_account.to_account_info(),
    //     to: ctx.accounts.user_token_account.to_account_info(),
    //     authority: ctx.accounts.gateway.to_account_info(),
    // };
    // let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    // token::transfer(cpi_ctx, output_amount)?;

    // // Emit event
    // emit!(EddyCrossChainReceive {
    //     external_id,
    //     from_token: ctx.accounts.from_token_account.mint,
    //     to_token: ctx
    //         .accounts
    //         .to_mint
    //         .as_ref()
    //         .map(|m| m.key())
    //         .unwrap_or(ctx.accounts.from_token_account.mint),
    //     amount,
    //     output_amount,
    //     wallet_address: ctx.accounts.user.key(),
    //     payload: message,
    // });

    Ok(())
}

// ABI decoder
fn decode_u256(data: &[u8], offset: &mut usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[*offset + 24..*offset + 32]);
    *offset += 32;
    u64::from_be_bytes(bytes)
}

fn decode_address(data: &[u8], offset: &mut usize) -> [u8; 20] {
    let mut address = [0u8; 20];
    address.copy_from_slice(&data[*offset + 12..*offset + 32]);
    *offset += 32;
    address
}

fn decode_bytes32(data: &[u8], offset: &mut usize) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&data[*offset..*offset + 32]);
    *offset += 32;
    bytes
}

fn decode_bytes(data: &[u8], offset: &mut usize) -> Vec<u8> {
    let len = decode_u256(data, offset) as usize;
    let mut bytes = vec![0u8; len];
    bytes.copy_from_slice(&data[*offset..*offset + len]);
    *offset += ((len + 31) / 32) * 32; // 32字节对齐
    bytes
}
