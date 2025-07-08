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

/// Encode accounts and data using ABI encoding similar to ethers
/// This function encodes the structure: tuple(tuple(bytes32 publicKey, bool isWritable)[] accounts, bytes data)
pub fn encode_abi_accounts_and_data(accounts: &[(Pubkey, bool)], data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();

    // Encode the outer tuple structure
    // First encode the accounts array
    let accounts_count = accounts.len() as u32;

    // Encode array length (32 bytes)
    let mut count_bytes = [0u8; 32];
    count_bytes[28..32].copy_from_slice(&accounts_count.to_be_bytes());
    encoded.extend_from_slice(&count_bytes);

    // Encode each account tuple (bytes32 publicKey, bool isWritable)
    for (pubkey, is_writable) in accounts {
        // Encode publicKey (bytes32) - pad to 32 bytes
        let mut pubkey_bytes = [0u8; 32];
        pubkey_bytes.copy_from_slice(&pubkey.to_bytes());
        encoded.extend_from_slice(&pubkey_bytes);

        // Encode isWritable (bool) - pad to 32 bytes
        let mut bool_bytes = [0u8; 32];
        bool_bytes[31] = if *is_writable { 1 } else { 0 };
        encoded.extend_from_slice(&bool_bytes);
    }

    // Encode the data bytes
    let data_length = data.len() as u32;

    // Encode data length (32 bytes)
    let mut data_len_bytes = [0u8; 32];
    data_len_bytes[28..32].copy_from_slice(&data_length.to_be_bytes());
    encoded.extend_from_slice(&data_len_bytes);

    // Encode data with padding to 32-byte boundary
    let padding_needed = (32 - (data.len() % 32)) % 32;
    encoded.extend_from_slice(data);
    encoded.extend_from_slice(&vec![0u8; padding_needed]);

    encoded
}

/// Decode ABI encoded accounts and data
pub fn decode_abi_accounts_and_data(encoded_data: &[u8]) -> Result<(Vec<(Pubkey, bool)>, Vec<u8>)> {
    if encoded_data.len() < 32 {
        return Err(GatewayError::InvalidInstructionData.into());
    }

    let mut offset = 0;

    // Decode accounts array length
    let mut count_bytes = [0u8; 4];
    count_bytes.copy_from_slice(&encoded_data[offset + 28..offset + 32]);
    let accounts_count = u32::from_be_bytes(count_bytes) as usize;
    offset += 32;

    // Decode accounts
    let mut accounts = Vec::new();
    for _ in 0..accounts_count {
        if offset + 64 > encoded_data.len() {
            return Err(GatewayError::InvalidInstructionData.into());
        }

        // Decode publicKey
        let pubkey_bytes = &encoded_data[offset..offset + 32];
        let pubkey = Pubkey::new_from_array(
            pubkey_bytes
                .try_into()
                .map_err(|_| GatewayError::InvalidInstructionData)?,
        );
        offset += 32;

        // Decode isWritable
        let is_writable = encoded_data[offset + 31] != 0;
        offset += 32;

        accounts.push((pubkey, is_writable));
    }

    // Decode data length
    if offset + 32 > encoded_data.len() {
        return Err(GatewayError::InvalidInstructionData.into());
    }
    let mut data_len_bytes = [0u8; 4];
    data_len_bytes.copy_from_slice(&encoded_data[offset + 28..offset + 32]);
    let data_length = u32::from_be_bytes(data_len_bytes) as usize;
    offset += 32;

    // Decode data
    if offset + data_length > encoded_data.len() {
        return Err(GatewayError::InvalidInstructionData.into());
    }
    let data = encoded_data[offset..offset + data_length].to_vec();

    Ok((accounts, data))
}
