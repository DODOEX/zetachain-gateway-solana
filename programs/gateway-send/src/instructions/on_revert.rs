use {
    crate::{
        errors::GatewayError,
        instructions::{decode_bytes32, SOL},
        states::{config::Config, events::EddyCrossChainRevert},
        utils::{decode_abi_accounts_and_data, encode_abi_accounts_and_data},
        CONFIG_SEED,
    },
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Token},
};

#[derive(Accounts)]
pub struct OnRevert<'info> {
    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    /// CHECK: must have
    pub gateway_pda: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn on_revert<'info>(
    ctx: Context<'_, '_, '_, 'info, OnRevert<'info>>,
    amount: u64,
    sender: Pubkey,
    data: Vec<u8>,
) -> Result<()> {
    let external_id = decode_bytes32(&data, &mut 0);
    let token = if ctx.remaining_accounts.len() == 1 {
        // check balance
        if ctx.accounts.config.to_account_info().lamports() < amount {
            return Err(GatewayError::InsufficientBalance.into());
        }
        // transfer sol
        ctx.accounts.config.sub_lamports(amount).unwrap();
        ctx.remaining_accounts[0].add_lamports(amount).unwrap();
        SOL
    } else if ctx.remaining_accounts.len() == 4 {
        // transfer token
        let cpi_accounts = token::Transfer {
            from: ctx.remaining_accounts[1].to_account_info(),
            to: ctx.remaining_accounts[2].to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let config_signer: &[&[&[u8]]] = &[&[CONFIG_SEED, &[ctx.bumps.config]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, config_signer);
        token::transfer(cpi_ctx, amount)?;
        ctx.remaining_accounts[3].key()
    } else {
        return Err(GatewayError::InvalidRemainingAccounts.into());
    };

    msg!(
        "EddyCrossChainRevert {} {} {} {}",
        hex::encode(external_id),
        token,
        amount,
        sender,
    );

    emit!(EddyCrossChainRevert {
        external_id,
        token,
        amount,
        wallet_address: sender,
    });

    Ok(())
}

/// Encode accounts and data for on_revert instruction call
/// This function prepares the ABI encoded data that can be used to call on_revert
pub fn encode_on_revert_call(
    config_pda: &Pubkey,
    gateway_pda: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
    amount: u64,
    sender: &Pubkey,
    external_id: [u8; 32],
    remaining_accounts: &[Pubkey],
) -> Vec<u8> {
    // Prepare base accounts for on_revert instruction
    let mut accounts = vec![
        (*config_pda, false),    // config account (not writable from caller perspective)
        (*gateway_pda, false),   // gateway_pda (not writable)
        (*token_program, false), // token_program (not writable)
        (*system_program, false), // system_program (not writable)
    ];

    // Add remaining accounts based on the type of transfer
    if remaining_accounts.len() == 1 {
        // SOL transfer: add recipient account as writable
        accounts.push((remaining_accounts[0], true));
    } else if remaining_accounts.len() == 4 {
        // Token transfer: add token accounts
        accounts.push((remaining_accounts[0], false)); // Some account (not writable)
        accounts.push((remaining_accounts[1], true)); // From account (writable)
        accounts.push((remaining_accounts[2], true)); // To account (writable)
        accounts.push((remaining_accounts[3], false)); // Mint account (not writable)
    }

    // Prepare instruction data
    let mut data = Vec::new();
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&sender.to_bytes());
    data.extend_from_slice(&external_id);

    // Encode using ABI encoding
    encode_abi_accounts_and_data(&accounts, &data)
}

/// Decode on_revert call data and extract parameters
pub fn decode_on_revert_call(
    encoded_data: &[u8],
) -> Result<(Vec<(Pubkey, bool)>, u64, Pubkey, [u8; 32])> {
    let (accounts, data) = decode_abi_accounts_and_data(encoded_data)?;

    if data.len() < 8 + 32 + 32 {
        return Err(GatewayError::InvalidInstructionData.into());
    }

    let mut offset = 0;

    // Decode amount (u64 = 8 bytes)
    let amount_bytes = &data[offset..offset + 8];
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    offset += 8;

    // Decode sender (32 bytes)
    let sender_bytes = &data[offset..offset + 32];
    let sender = Pubkey::new_from_array(sender_bytes.try_into().unwrap());
    offset += 32;

    // Decode external_id (32 bytes)
    let external_id_bytes = &data[offset..offset + 32];
    let external_id = external_id_bytes.try_into().unwrap();
    offset += 32;

    Ok((accounts, amount, sender, external_id))
}
