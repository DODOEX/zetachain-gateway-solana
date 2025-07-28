use {
    crate::{
        errors::GatewayError, states::{config::Config, events::EddyCrossChainSend}, utils::{prepare_account_metas, prepare_account_metas_only_gateway}, AUTHORITY_SEED, CONFIG_SEED
    },
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, program::invoke_signed},
    },
    anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}},
};

/// Deposit fee used when depositing SOL or SPL tokens.
pub const DEPOSIT_FEE: u64 = 2_000_000;
pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

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

    /// CHECK: gateway is validated by the config account, which ensures it matches the expected gateway program
    #[account(address = config.gateway)]
    pub gateway: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_sol_and_call(
    ctx: Context<DepositSolAndCall>,
    target_contract: [u8; 20],
    amount: u64,
    dst_chain_id: u32,
    mut payload: Vec<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.global_nonce += 1;
    let user = &ctx.accounts.user;
    let receiver = payload[payload.len()-20..].to_vec();

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

    // Abort message is the external id and receiver
    let mut revert_message = external_id.to_vec();
    revert_message.extend_from_slice(&receiver);
    // let revert_message = encode_on_revert_call(
    //     &ctx.accounts.config.key(),
    //     &ctx.accounts.gateway.key(),
    //     &Token::id(),
    //     &ctx.accounts.system_program.key(),
    //     amount,
    //     &user.key(),
    //     external_id,
    //     &account_metas.iter().map(|meta| meta.pubkey).collect::<Vec<_>>(),
    // );

    // Prepare data
    let mut data = [65, 33, 186, 198, 114, 223, 133, 57].to_vec(); // deposit_and_call
    let args = DepositAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload.clone(),
        revert_options: Some(RevertOptions {
            revert_address: *ctx.program_id,
            abort_address: target_contract,
            call_on_revert: true,
            revert_message,
            on_revert_gas_limit: ctx.accounts.config.gas_limit,
        }),
        deposit_fee: DEPOSIT_FEE,
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

    msg!("EddyCrossChainSend 0x{} {} {} {} {} {} {} 0x{}", 
        hex::encode(external_id),
        dst_chain_id,
        SOL_MINT,
        SOL_MINT,
        amount,
        amount,
        user.key(),
        hex::encode(&payload),
    );

    // Emit event
    emit!(EddyCrossChainSend {
        external_id,
        dst_chain_id,
        from_token: SOL_MINT,
        to_token: SOL_MINT,
        amount,
        output_amount: amount,
        wallet_address: user.key(),
        payload,
    });

    Ok(())
}


#[derive(Accounts)]
pub struct DepositSplAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut, seeds = [AUTHORITY_SEED], bump)]
    pub program_authority: SystemAccount<'info>,

    #[account(
        constraint = asset_mint.key() == user_token_account.mint,
    )]
    pub asset_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,     
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = asset_mint,
        associated_token::authority = program_authority,
        associated_token::token_program = token_program,
    )]
    pub program_token_account: Account<'info, TokenAccount>,

    /// CHECK: gateway is validated by the config account, which ensures it matches the expected gateway program
    #[account(address = config.gateway)]
    pub gateway: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_spl_and_call<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositSplAndCall<'info>>,
    target_contract: [u8; 20],
    amount: u64,
    asset: Pubkey,
    dst_chain_id: u32,
    mut payload: Vec<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.global_nonce += 1;
    let user = &ctx.accounts.user;
    let receiver = payload[payload.len()-20..].to_vec();

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;
    // External id is the first 32 bytes of the payload
    payload.splice(0..0, external_id.to_vec());

    // Transfer deposit fee sols from user to program
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &user.key(),
        &ctx.accounts.program_authority.key(),
        DEPOSIT_FEE,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            user.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
        ],
    )?;

    // Transfer spl token from user to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.program_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Prepare account metas for gateway call
    // remaining_accounts should contain: [gateway_meta, whitelisted_entry, to_account]
    let account_metas = vec![
        AccountMeta::new(ctx.accounts.program_authority.key(), true),
        AccountMeta::new(ctx.remaining_accounts[0].key(), false), // gateway_meta
        AccountMeta::new(ctx.remaining_accounts[1].key(), false), // whitelisted_entry
        AccountMeta::new_readonly(ctx.accounts.asset_mint.key(), false), // asset_mint - use asset parameter
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false), // token_program - use ctx.accounts
        AccountMeta::new(ctx.accounts.program_token_account.key(), false), // program_token_account - use ctx.accounts
        AccountMeta::new(ctx.remaining_accounts[2].key(), false), // to_account
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false), // system_program - use ctx.accounts
    ];

    let mut revert_message = external_id.to_vec();
    revert_message.extend_from_slice(&receiver);

    // Prepare data
    let mut data = [14, 181, 27, 187, 171, 61, 237, 147].to_vec(); // deposit_spl_token_and_call
    let args = DepositAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload.clone(),
        revert_options: Some(RevertOptions {
            revert_address: *ctx.program_id,
            abort_address: target_contract,
            call_on_revert: true,
            revert_message,
            on_revert_gas_limit: ctx.accounts.config.gas_limit,
        }),
        deposit_fee: DEPOSIT_FEE,
    };
    data.extend(args.try_to_vec()?);

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: account_metas.clone(),
        data,
    };

    // Prepare all accounts for gateway call in the same order as account_metas
    let all_accounts = vec![
        ctx.accounts.program_authority.to_account_info(), // program_authority
        ctx.remaining_accounts[0].clone(), // gateway_meta
        ctx.remaining_accounts[1].clone(), // whitelisted_entry
        ctx.accounts.asset_mint.to_account_info(), // asset_mint
        ctx.accounts.token_program.to_account_info(), // token_program
        ctx.accounts.program_token_account.to_account_info(), // program_token_account
        ctx.remaining_accounts[2].clone(), // to_account
        ctx.accounts.system_program.to_account_info(), // system_program
    ];
    
    invoke_signed(
        &gateway_ix,
        &all_accounts,
        &[&[AUTHORITY_SEED, &[ctx.bumps.program_authority]]],
    )?;

    msg!("EddyCrossChainSend 0x{} {} {} {} {} {} {} 0x{}", 
        hex::encode(external_id),
        dst_chain_id,
        asset,
        asset,
        amount,
        amount,
        user.key(),
        hex::encode(&payload),
    );

    // Emit event
    emit!(EddyCrossChainSend {
        external_id,
        dst_chain_id,
        from_token: asset,
        to_token: asset,
        amount,
        output_amount: amount,
        wallet_address: user.key(),
        payload,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DepositSplSwapSplAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut, seeds = [AUTHORITY_SEED], bump)]
    pub program_authority: SystemAccount<'info>,

    #[account(
        mut, 
        token::authority = user,
        token::token_program = token_program,
        constraint = user_from_token_account.mint != asset_mint.key()
    )]
    pub user_from_token_account: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed, payer = user, associated_token::mint = user_from_token_account, associated_token::authority = program_authority, associated_token::token_program = token_program)]
    pub program_from_token_account: Account<'info, TokenAccount>,

    #[account(mint::token_program = token_program)]
    pub asset_mint: Box<Account<'info, Mint>>,

    #[account(init_if_needed, payer = user, associated_token::mint = asset_mint, associated_token::authority = program_authority, associated_token::token_program = token_program)]
    pub program_asset_token_account: Account<'info, TokenAccount>,

    /// CHECK: dodo_route_proxy is validated by the config account, which ensures it matches the expected dodo route proxy program
    #[account(address = config.dodo_route_proxy)]
    pub dodo_route_proxy: UncheckedAccount<'info>,

    /// CHECK: gateway is validated by the config account, which ensures it matches the expected gateway program
    #[account(address = config.gateway)]
    pub gateway: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// SPL token swap to another SPL token
pub fn deposit_spl_swap_spl_and_call<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositSplSwapSplAndCall<'info>>,
    target_contract: [u8; 20],
    amount: u64,
    swap_data: Vec<u8>,
    asset: Pubkey,
    dst_chain_id: u32,
    mut payload: Vec<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.global_nonce += 1;
    let user = &ctx.accounts.user;
    let receiver = payload[payload.len()-20..].to_vec();

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;
    // External id is the first 32 bytes of the payload
    payload.splice(0..0, external_id.to_vec());

    // Transfer deposit fee sols from user to program
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &user.key(),
        &ctx.accounts.program_authority.key(),
        DEPOSIT_FEE,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            user.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
        ],
    )?;

    // Transfer tokens from user to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_from_token_account.to_account_info(),
        to: ctx.accounts.program_from_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Prepare account metas for DODO swap
    let (gateway_account_metas, route_proxy_account_metas) =
        prepare_account_metas(ctx.remaining_accounts, user, &ctx.accounts.dodo_route_proxy.key())?;

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
    
    // Get swap result
    let result_account = &ctx.remaining_accounts[0];
    let result_data = result_account.try_borrow_data()?;
    let output_amount = u64::from_le_bytes(result_data[..8].try_into().unwrap());

    // Prepare account metas for gateway call (similar to deposit_spl_and_call)
    let account_metas = vec![
        AccountMeta::new(ctx.accounts.program_authority.key(), true),
        AccountMeta::new(ctx.remaining_accounts[route_proxy_account_metas.len() + 1].key(), false), // gateway_meta
        AccountMeta::new(ctx.remaining_accounts[route_proxy_account_metas.len() + 2].key(), false), // whitelisted_entry
        AccountMeta::new_readonly(ctx.accounts.asset_mint.key(), false), // asset_mint
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false), // token_program
        AccountMeta::new(ctx.accounts.program_asset_token_account.key(), false), // program_asset_token_account
        AccountMeta::new(ctx.remaining_accounts[route_proxy_account_metas.len() + 3].key(), false), // to_account
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false), // system_program
    ];

    // Prepare revert message (similar to deposit_spl_and_call)
    let mut revert_message = external_id.to_vec();
    revert_message.extend_from_slice(&receiver);

    // Prepare data for gateway call
    let mut data = [14, 181, 27, 187, 171, 61, 237, 147].to_vec(); // deposit_spl_token_and_call
    let args = DepositAndCallArgs {
        amount: output_amount, // Use swapped amount
        receiver: target_contract,
        message: payload.clone(),
        revert_options: Some(RevertOptions {
            revert_address: *ctx.program_id,
            abort_address: target_contract,
            call_on_revert: true,
            revert_message,
            on_revert_gas_limit: ctx.accounts.config.gas_limit,
        }),
        deposit_fee: DEPOSIT_FEE,
    };
    data.extend(args.try_to_vec()?);

    // Call Gateway's deposit_and_call
    let gateway_ix = Instruction {
        program_id: ctx.accounts.gateway.key(),
        accounts: account_metas.clone(),
        data,
    };

    // Prepare all accounts for gateway call in the same order as account_metas
    let all_accounts = vec![
        ctx.accounts.program_authority.to_account_info(), // program_authority
        ctx.remaining_accounts[route_proxy_account_metas.len() + 1].clone(), // gateway_meta
        ctx.remaining_accounts[route_proxy_account_metas.len() + 2].clone(), // whitelisted_entry
        ctx.accounts.asset_mint.to_account_info(), // asset_mint
        ctx.accounts.token_program.to_account_info(), // token_program
        ctx.accounts.program_asset_token_account.to_account_info(), // program_asset_token_account
        ctx.remaining_accounts[route_proxy_account_metas.len() + 3].clone(), // to_account
        ctx.accounts.system_program.to_account_info(), // system_program
    ];
    
    invoke_signed(
        &gateway_ix,
        &all_accounts,
        &[&[AUTHORITY_SEED, &[ctx.bumps.program_authority]]],
    )?;

    msg!("EddyCrossChainSend 0x{} {} {} {} {} {} {} 0x{}", 
        hex::encode(external_id),
        dst_chain_id,
        ctx.accounts.user_from_token_account.mint,
        asset,
        amount,
        output_amount,
        user.key(),
        hex::encode(&payload),
    );

    // Emit event
    emit!(EddyCrossChainSend {
        external_id,
        dst_chain_id,
        from_token: ctx.accounts.user_from_token_account.mint,
        to_token: asset,
        amount,
        output_amount,
        wallet_address: user.key(),
        payload,
    });

    Ok(())
}


#[derive(Accounts)]
pub struct DepositSplSwapSolAndCall<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut, seeds = [AUTHORITY_SEED], bump)]
    pub program_authority: SystemAccount<'info>,

    #[account(
        mut,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_from_token_account: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed, payer = user, associated_token::mint = user_from_token_account, associated_token::authority = program_authority, associated_token::token_program = token_program)]
    pub program_from_token_account: Account<'info, TokenAccount>,

    /// CHECK: dodo_route_proxy is validated by the config account, which ensures it matches the expected dodo route proxy program
    #[account(address = config.dodo_route_proxy)]
    pub dodo_route_proxy: UncheckedAccount<'info>,

    /// CHECK: gateway is validated by the config account, which ensures it matches the expected gateway program
    #[account(address = config.gateway)]
    pub gateway: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// SPL token swap to SOL
pub fn deposit_spl_swap_sol_and_call<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositSplSwapSolAndCall<'info>>,
    target_contract: [u8; 20],
    amount: u64,
    swap_data: Vec<u8>,
    asset: Pubkey,
    dst_chain_id: u32,
    mut payload: Vec<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.global_nonce += 1;
    let user = &ctx.accounts.user;
    let receiver = payload[payload.len()-20..].to_vec();

    // Calculate external_id
    let external_id = calc_external_id(ctx.program_id, &user.key(), config.global_nonce)?;
    // External id is the first 32 bytes of the payload
    payload.splice(0..0, external_id.to_vec());

    // Transfer deposit fee sols from user to program
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &user.key(),
        &ctx.accounts.program_authority.key(),
        DEPOSIT_FEE,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            user.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
        ],
    )?;

    // Transfer tokens from user to program
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_from_token_account.to_account_info(),
        to: ctx.accounts.program_from_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Prepare account metas for DODO swap
    let(gateway_account_metas, route_proxy_account_metas) =
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
    
    // Get swap result
    let result_account = &ctx.remaining_accounts[0];
    let result_data = result_account.try_borrow_data()?;
    let output_amount = u64::from_le_bytes(result_data[..8].try_into().unwrap());

    // Prepare account metas for gateway call (similar to deposit_sol_and_call)
    let account_metas = prepare_account_metas_only_gateway(&ctx.remaining_accounts[route_proxy_account_metas.len() + 1..], user)?;

    // Prepare revert message (similar to deposit_sol_and_call)
    let mut revert_message = external_id.to_vec();
    revert_message.extend_from_slice(&receiver);

    // Prepare data for gateway call
    let mut data = [65, 33, 186, 198, 114, 223, 133, 57].to_vec(); // deposit_and_call
    let args = DepositAndCallArgs {
        amount: output_amount, // Use swapped amount
        receiver: target_contract,
        message: payload.clone(),
        revert_options: Some(RevertOptions {
            revert_address: *ctx.program_id,
            abort_address: target_contract,
            call_on_revert: true,
            revert_message,
            on_revert_gas_limit: ctx.accounts.config.gas_limit,
        }),
        deposit_fee: DEPOSIT_FEE,
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
        &ctx.remaining_accounts[route_proxy_account_metas.len() + 1..],
        &[&[AUTHORITY_SEED, &[ctx.bumps.program_authority]]],
    )?;

    msg!("EddyCrossChainSend 0x{} {} {} {} {} {} {} 0x{}", 
        hex::encode(external_id),
        dst_chain_id,
        ctx.accounts.user_from_token_account.mint,
        SOL_MINT,
        amount,
        output_amount,
        user.key(),
        hex::encode(&payload),
    );

    // Emit event
    emit!(EddyCrossChainSend {
        external_id,
        dst_chain_id,
        from_token: ctx.accounts.user_from_token_account.mint,
        to_token: SOL_MINT,
        amount,
        output_amount,
        wallet_address: user.key(),
        payload,
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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositArgs {
    pub amount: u64,
    pub receiver: [u8; 20],
    pub revert_options: Option<RevertOptions>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositAndCallArgs {
    pub amount: u64,
    pub receiver: [u8; 20],
    pub message: Vec<u8>,
    pub revert_options: Option<RevertOptions>,
    pub deposit_fee: u64,
}

pub type DepositSplAndCallArgs = DepositAndCallArgs;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RevertOptions {
    pub revert_address: Pubkey,
    pub abort_address: [u8; 20],
    pub call_on_revert: bool,
    pub revert_message: Vec<u8>,
    pub on_revert_gas_limit: u64,
}