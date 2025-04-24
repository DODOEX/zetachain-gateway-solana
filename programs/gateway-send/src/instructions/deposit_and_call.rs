use {
    crate::{
        errors::GatewayError, states::{config::Config, events::EddyCrossChainSwap}, utils::{prepare_account_metas, prepare_account_metas_only_gateway}, AUTHORITY_SEED, CONFIG_SEED
    },
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, program::invoke_signed},
    },
    anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer},
};

/// Deposit fee used when depositing SOL or SPL tokens.
pub const DEPOSIT_FEE: u64 = 2_000_000;

#[derive(Accounts)]
pub struct DepositSolAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

     #[account(mut, seeds = [AUTHORITY_SEED], bump)]
    pub program_authority: SystemAccount<'info>,

    #[account(address = config.gateway)]
    pub gateway: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_sol_and_call(
    ctx: Context<DepositSolAndCall>,
    amount: u64,
    target_contract: [u8; 20],
    mut payload: Vec<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.global_nonce += 1;
    let user = &ctx.accounts.user;

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;
    // External id is the first 32 bytes of the payload
    payload.splice(0..0, external_id.to_vec());

    // Transfer sols from user to program
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &user.key(),
        &ctx.accounts.program_authority.key(),
        amount + DEPOSIT_FEE,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            user.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
        ],
    )?;

    // Prepare account metas
    let account_metas = prepare_account_metas_only_gateway(ctx.remaining_accounts, user)?;

    // Prepare data
    let mut data = [65, 33, 186, 198, 114, 223, 133, 57].to_vec(); // deposit_and_call
    let args = DepositAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload,
        revert_options: Some(RevertOptions {
            revert_address: *ctx.program_id,
            abort_address: Pubkey::default(),
            call_on_revert: true,
            revert_message: Vec::new(),
            on_revert_gas_limit: 0,
        }),
    };
    data.extend(args.try_to_vec()?);

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: account_metas.clone(),
        data,
    };

    invoke_signed(
        &gateway_ix,
        ctx.remaining_accounts,
        &[&[AUTHORITY_SEED, &[ctx.bumps.program_authority]]],
    )?;

    // Emit event
    emit!(EddyCrossChainSwap {
        external_id,
        from_token: Pubkey::default(),
        to_token: Pubkey::default(),
        amount,
        output_amount: amount,
        wallet_address: user.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DepositAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,     
        token::authority = user,
        token::token_program = token_program,
    )]
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
    let account_metas = prepare_account_metas_only_gateway(ctx.remaining_accounts, &user)?;

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: account_metas.clone(),
        data: payload,
    };

    invoke_signed(
        &gateway_ix,
        &ctx.remaining_accounts[account_metas.len() + 2..],
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
        seeds = [CONFIG_SEED],
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
        prepare_account_metas(ctx.remaining_accounts, user, &ctx.accounts.gateway.key())?;

    // Call DODO Route Proxy for token swap
    let swap_ix = Instruction {
        program_id: ctx.accounts.dodo_route_proxy.key(),
        accounts: route_proxy_account_metas.clone(),
        data: swap_data,
    };
    invoke_signed(
        &swap_ix,
        &ctx.remaining_accounts[1..route_proxy_account_metas.len() + 1], // remaining_accounts[0] is the swap result account
        &[],
    )
    .map_err(|_| GatewayError::RouteProxyCallFailed)?;
    let result_account = &ctx.remaining_accounts[0];
    let result_data = result_account.try_borrow_data()?;
    let result = u64::from_le_bytes(result_data[..8].try_into().unwrap());

    // Decode output amount
    let output_amount = result;

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

// #[derive(Accounts)]
// pub struct OnCall<'info> {
//     #[account(mut, seeds = [CONNECTED_SEED], bump)]
//     pub pda: Account<'info, ConnectedPda>,

//     /// CHECK: This is test program.
//     pub gateway_pda: UncheckedAccount<'info>,

//     /// CHECK: This is test program.
//     pub random_wallet: UncheckedAccount<'info>,

//     pub system_program: Program<'info, System>,
// }

// pub fn on_call(
//     ctx: Context<OnCall>,
//     amount: u64,
//     sender: [u8; 20],
//     data: Vec<u8>,
// ) -> Result<()> {
//     let pda = &mut ctx.accounts.pda;

//     // Store the sender's public key
//     pda.last_sender = sender;

//     // Convert data to a string and store it
//     let message = String::from_utf8(data).map_err(|_| OnRevertError::InvalidDataFormat)?;
//     pda.last_message = message;

//     // Transfer some portion of lamports transferred from gateway to another account
//     pda.sub_lamports(amount / 2)?;
//     ctx.accounts.random_wallet.add_lamports(amount / 2)?;

//     // Check if the message contains "revert" and return an error if so
//     if pda.last_message.contains("revert") {
//         msg!(
//             "Reverting transaction due to message: '{}'",
//             pda.last_message
//         );
//         return Err(OnRevertError::RevertMessage.into());
//     }

//     msg!(
//         "On call executed with amount {}, sender {:?} and message {}",
//         amount,
//         pda.last_sender,
//         pda.last_message
//     );

//     Ok(())
// }


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

#[derive(AnchorSerialize, AnchorDeserialize)]
struct DepositAndCallArgs {
    pub amount: u64,
    pub receiver: [u8; 20],
    pub message: Vec<u8>,
    pub revert_options: Option<RevertOptions>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
struct RevertOptions {
    pub revert_address: Pubkey,
    pub abort_address: Pubkey,
    pub call_on_revert: bool,
    pub revert_message: Vec<u8>,
    pub on_revert_gas_limit: u64,
}