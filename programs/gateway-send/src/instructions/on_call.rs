use {
    crate::{
        errors::GatewayError,
        states::{config::Config, events::EddyCrossChainReceive},
        CONFIG_SEED,
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::{self},
        token::{self, Token},
    },
    std::str::FromStr,
};

pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111111");

#[derive(Accounts)]
pub struct OnCall<'info> {
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

/*
remaining_accounts: [
    user_wallet,
    program_token_account,
    user_token_account,
    token_mint,
]
 */
pub fn on_call<'info>(
    ctx: Context<'_, '_, 'info, 'info, OnCall<'info>>,
    amount: u64,
    _sender: [u8; 20],
    data: Vec<u8>,
) -> Result<()> {
    let mut offset = 0;
    let external_id = decode_bytes32(&data, &mut offset);
    let _output_amount = decode_u256(&data, &mut offset);
    let receiver_len = decode_u16(&data, &mut offset);
    let swap_data_len = decode_u16(&data, &mut offset);
    let receiver_bytes = decode_bytes_with_length(&data, &mut offset, receiver_len as usize);
    let receiver_str = String::from_utf8(receiver_bytes).map_err(|_| GatewayError::InvalidUtf8)?;
    let receiver = Pubkey::from_str(&receiver_str).map_err(|_| GatewayError::InvalidPubkey)?;
    let swap_data = decode_bytes_with_length(&data, &mut offset, swap_data_len as usize);

    // check receiver account
    if ctx.remaining_accounts[0].key() != receiver {
        return Err(GatewayError::InvalidReceiverAccount.into());
    }
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
        // Check SPL token balance
        let from_token_account =
            Account::<token::TokenAccount>::try_from(&ctx.remaining_accounts[1])?;
        if from_token_account.amount < amount {
            return Err(GatewayError::InsufficientBalance.into());
        }
        // Check if the 'to' account exists, if not, create it
        let to_account_info = ctx.remaining_accounts[2].to_account_info();
        if to_account_info.owner != &token::ID || to_account_info.data_is_empty() {
            // Create associated token account
            let payer = ctx.accounts.config.to_account_info();
            let mint = ctx.remaining_accounts[3].to_account_info();
            let authority = ctx.remaining_accounts[0].to_account_info();
            let ata_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                associated_token::Create {
                    payer,
                    associated_token: to_account_info.clone(),
                    authority,
                    mint,
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            );
            associated_token::create(ata_ctx)?;
        }
        // transfer token
        let cpi_accounts = token::Transfer {
            from: ctx.remaining_accounts[1].to_account_info(),
            to: to_account_info,
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
        "EddyCrossChainReceive {} {} {} {}",
        hex::encode(external_id),
        token,
        receiver,
        amount
    );
    emit!(EddyCrossChainReceive {
        external_id,
        from_token: token,
        to_token: token,
        amount,
        output_amount: amount,
        wallet_address: receiver,
        payload: swap_data,
    });

    Ok(())
}

// ABI decoder
pub fn decode_u256(data: &[u8], offset: &mut usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[*offset + 24..*offset + 32]);
    *offset += 32;
    u64::from_be_bytes(bytes)
}

pub fn decode_address(data: &[u8], offset: &mut usize) -> [u8; 20] {
    let mut address = [0u8; 20];
    address.copy_from_slice(&data[*offset + 12..*offset + 32]);
    *offset += 32;
    address
}

pub fn decode_bytes32(data: &[u8], offset: &mut usize) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&data[*offset..*offset + 32]);
    *offset += 32;
    bytes
}

pub fn decode_bytes(data: &[u8], offset: &mut usize) -> Vec<u8> {
    let len = decode_u256(data, offset) as usize;
    let mut bytes = vec![0u8; len];
    bytes.copy_from_slice(&data[*offset..*offset + len]);
    *offset += ((len + 31) / 32) * 32; // 32字节对齐
    bytes
}

pub fn decode_u16(data: &[u8], offset: &mut usize) -> u16 {
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(&data[*offset..*offset + 2]);
    *offset += 2;
    u16::from_be_bytes(bytes)
}

pub fn decode_bytes_with_length(data: &[u8], offset: &mut usize, length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    bytes.copy_from_slice(&data[*offset..*offset + length]);
    *offset += length;
    bytes
}
