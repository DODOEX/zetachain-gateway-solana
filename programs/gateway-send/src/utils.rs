use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::AccountMeta;

use crate::errors::GatewayError;

/// Prepares account metas for withdraw and call, revert if unallowed account is passed
#[allow(clippy::collapsible_else_if)]
pub fn prepare_account_metas(
    remaining_accounts: &[AccountInfo],
    signer: &Signer,
    gateway: &Pubkey,
) -> Result<(Vec<AccountMeta>, Vec<AccountMeta>)> {
    require!(
        remaining_accounts.len() > 0,
        GatewayError::InvalidInstructionData
    );
    let mut route_proxy_account_metas = Vec::new();
    let mut gateway_account_metas = Vec::new();

    let mut now_route_proxy = true;
    // Skip first account as it is the route proxy result account
    for account_info in remaining_accounts[1..].iter() {
        let account_key = account_info.key;

        // Prevent signer from being included
        require!(
            account_key != signer.key,
            GatewayError::InvalidInstructionData
        );

        // Gateway key is split between route proxy and gateway
        if account_key == gateway {
            now_route_proxy = false;
        } else if account_info.is_writable {
            if now_route_proxy {
                route_proxy_account_metas.push(AccountMeta::new(*account_key, false));
            } else {
                gateway_account_metas.push(AccountMeta::new(*account_key, false));
            }
        } else {
            if now_route_proxy {
                route_proxy_account_metas.push(AccountMeta::new_readonly(*account_key, false));
            } else {
                gateway_account_metas.push(AccountMeta::new_readonly(*account_key, false));
            }
        }
    }
    Ok((route_proxy_account_metas, gateway_account_metas))
}

pub fn prepare_account_metas_only_gateway(
    remaining_accounts: &[AccountInfo],
    signer: &Signer,
) -> Result<Vec<AccountMeta>> {
    require!(
        !remaining_accounts.is_empty(),
        GatewayError::InvalidInstructionData
    );
    let mut account_metas = Vec::new();

    for (i, account_info) in remaining_accounts.iter().enumerate() {
        let account_key = account_info.key;

        // Prevent signer from being included
        require!(
            account_key != signer.key,
            GatewayError::InvalidInstructionData
        );
        let mut is_signer = account_info.is_signer;
        // first account is signer
        if i == 0 {
            is_signer = true;
        }
        if account_info.is_writable {
            account_metas.push(AccountMeta::new(*account_key, is_signer));
        } else {
            account_metas.push(AccountMeta::new_readonly(*account_key, is_signer));
        }
    }
    Ok(account_metas)
}
pub fn prepare_account_metas_only_route_proxy(
    remaining_accounts: &[AccountInfo],
    signer: &Signer,
) -> Result<Vec<AccountMeta>> {
    require!(
        remaining_accounts.len() > 0,
        GatewayError::InvalidInstructionData
    );
    let mut account_metas = Vec::new();

    // Skip first account as it is the route proxy result account
    for account_info in remaining_accounts[1..].iter() {
        let account_key = account_info.key;

        // Prevent signer from being included
        require!(
            account_key != signer.key,
            GatewayError::InvalidInstructionData
        );
        let is_signer = account_info.is_signer;
        if account_info.is_writable {
            account_metas.push(AccountMeta::new(*account_key, is_signer));
        } else {
            account_metas.push(AccountMeta::new_readonly(*account_key, is_signer));
        }
    }
    Ok(account_metas)
}
