use anchor_client::{
    anchor_lang::{prelude::AccountMeta, AnchorSerialize, Discriminator},
    solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program},
};
use anchor_spl::{associated_token::spl_associated_token_account, token};
use anyhow::Result;
use gateway_send::{
    gateway_send::{DepositAndCallArgs, DepositArgs, DepositSplAndCallArgs, RevertOptions},
    instructions::{
        on_revert::{decode_on_revert_call, encode_on_revert_call},
        DEPOSIT_FEE,
    },
    utils::{decode_abi_accounts_and_data, encode_abi_accounts_and_data},
    AUTHORITY_SEED, CONFIG_SEED,
};

use crate::{read_keypair_file, ClientConfig};

pub fn create_config_instr(
    config: &ClientConfig,
    gateway: Pubkey,
    dodo_route_proxy: Pubkey,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::CreateConfig {
        gateway,
        dodo_route_proxy,
    };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::CreateConfig::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn update_gateway_instr(config: &ClientConfig, gateway: Pubkey) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::UpdateGateway { gateway };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::UpdateGateway::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn update_dodo_route_proxy_instr(
    config: &ClientConfig,
    dodo_route_proxy: Pubkey,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::UpdateDodoRouteProxy { dodo_route_proxy };
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::UpdateDodoRouteProxy::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn update_gas_limit_instr(
    config: &ClientConfig,
    new_gas_limit: u64,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::UpdateGasLimit { new_gas_limit };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::UpdateGasLimit::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn update_owner_instr(config: &ClientConfig, new_owner: Pubkey) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::UpdateOwner { new_owner };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::UpdateOwner::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn close_config_instr(config: &ClientConfig) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::CloseConfig {};

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::CloseConfig::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn deposit_sol_and_call_instr(
    config: &ClientConfig,
    target_contract: [u8; 20],
    amount: u64,
    dst_chain_id: u32,
    payload: Vec<u8>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let (program_authority, _) = Pubkey::find_program_address(&[AUTHORITY_SEED], &program_id);

    let ix_data = gateway_send::instruction::DepositSolAndCall {
        target_contract,
        amount,
        dst_chain_id,
        payload,
    };

    let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new(program_authority, false),
            AccountMeta::new(config.gateway_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
            // remaining accounts, gateway deposit with call accounts
            AccountMeta::new(program_authority, false),
            AccountMeta::new(gateway_meta, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::DepositSolAndCall::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn deposit_spl_and_call_instr(
    config: &ClientConfig,
    target_contract: [u8; 20],
    amount: u64,
    asset: Pubkey,
    dst_chain_id: u32,
    payload: Vec<u8>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let (program_authority, _) = Pubkey::find_program_address(&[AUTHORITY_SEED], &program_id);

    let ix_data = gateway_send::instruction::DepositSplAndCall {
        target_contract,
        amount,
        asset,
        dst_chain_id,
        payload,
    };

    let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);
    let (whitelisted_entry, _) =
        Pubkey::find_program_address(&[b"whitelist", asset.as_ref()], &config.gateway_program);
    let user_account =
        spl_associated_token_account::get_associated_token_address(&payer.pubkey(), &asset);
    let program_account =
        spl_associated_token_account::get_associated_token_address(&program_authority, &asset);
    let to_account =
        spl_associated_token_account::get_associated_token_address(&gateway_meta, &asset);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new(program_authority, false),
            AccountMeta::new(asset, false),
            AccountMeta::new(user_account, false),
            AccountMeta::new(program_account, false),
            AccountMeta::new(config.gateway_program, false),
            AccountMeta::new(token::ID, false),
            AccountMeta::new_readonly(system_program::id(), false),
            // remaining accounts, gateway deposit with call accounts
            AccountMeta::new(gateway_meta, false),
            AccountMeta::new(whitelisted_entry, false),
            AccountMeta::new(to_account, false),
        ],
        data: {
            let mut data = gateway_send::instruction::DepositSplAndCall::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    println!("instruction: {:?}", instruction.accounts);
    panic!();
    Ok(vec![instruction])
}

// pub fn deposit_and_call_instr(
//     config: &ClientConfig,
//     dst_chain_id: u32,
//     amount: u64,
//     target_contract: [u8; 20],
//     receiver: [u8; 20],
//     payload: Vec<u8>,
//     external_id: Option<[u8; 32]>,
// ) -> Result<Vec<Instruction>> {
//     let payer = read_keypair_file(&config.payer_path)?;
//     let program_id = config.gateway_send_program;

//     let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
//     let ix_data = gateway_send::instruction::DepositSolAndCall {
//         target_contract,
//         amount,
//         dst_chain_id,
//         payload,
//     };

//     let instruction = Instruction {
//         program_id,
//         accounts: vec![
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new(config_pda, false),
//             AccountMeta::new_readonly(system_program::id(), false),
//         ],
//         data: {
//             let mut data = gateway_send::instruction::DepositAndCall::DISCRIMINATOR.to_vec();
//             data.extend(ix_data.try_to_vec().unwrap());
//             data
//         },
//     };
//     Ok(vec![instruction])
// }

// pub fn deposit_swap_and_call_instr(
//     config: &ClientConfig,
//     amount: u64,
//     swap_data: Vec<u8>,
//     target_contract: Pubkey,
//     asset: Pubkey,
//     payload: Vec<u8>,
// ) -> Result<Vec<Instruction>> {
//     let payer = read_keypair_file(&config.payer_path)?;
//     let program_id = config.gateway_send_program;

//     let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
//     let ix_data = gateway_send::instruction::DepositSwapAndCall {
//         amount,
//         swap_data,
//         target_contract,
//         asset,
//         payload,
//     };

//     let instruction = Instruction {
//         program_id,
//         accounts: vec![
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new(config_pda, false),
//             AccountMeta::new_readonly(system_program::id(), false),
//         ],
//         data: {
//             let mut data = gateway_send::instruction::DepositSwapAndCall::DISCRIMINATOR.to_vec();
//             data.extend(ix_data.try_to_vec().unwrap());
//             data
//         },
//     };
//     Ok(vec![instruction])
// }

pub fn deposit_sol_gateway_instr(
    config: &ClientConfig,
    amount: u64,
    receiver: [u8; 20],
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_program;

    let ix_data = DepositArgs {
        amount,
        receiver,
        revert_options: Some(RevertOptions {
            revert_address: config.gateway_send_program,
            abort_address: receiver,
            call_on_revert: true,
            revert_message: hex::decode("0x4B37ff61e17DdcD4cEA80AF768de9455FC373764").unwrap(),
            on_revert_gas_limit: 0,
        }),
    };

    let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(gateway_meta, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = [242, 35, 198, 137, 82, 225, 242, 182].to_vec(); // deposit
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn deposit_sol_and_call_gateway_instr(
    config: &ClientConfig,
    amount: u64,
    target_contract: [u8; 20],
    receiver: [u8; 20],
    payload: Vec<u8>,
    external_id: Option<[u8; 32]>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_program;

    // 如果external_id为None，则随机生成一个32字节的external_id
    let external_id = external_id.unwrap_or(rand::random());
    let mut payload = payload;
    payload.splice(0..0, external_id.to_vec());

    let ix_data = DepositAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload,
        revert_options: Some(RevertOptions {
            revert_address: config.gateway_send_program,
            abort_address: receiver,
            call_on_revert: true,
            revert_message: hex::decode("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000000043c86a896f9ea09859efb5693feb4e9252d436ceb03946619b2031c43933078d9000000000000000000000000000000000000000000000000000000000000000118a14c1ff4fdcdb919aadb9fc2340cc5047960db89930154409cccdf9a65bb42000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040d2638ca121027536f80b69093e83eba68b3ff11f9253af514b29affbd7bfa1c5ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace2").unwrap(),
            on_revert_gas_limit: 10000000,
        }),
        deposit_fee: DEPOSIT_FEE,
    };

    let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(gateway_meta, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = [65, 33, 186, 198, 114, 223, 133, 57].to_vec(); // deposit_and_call
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn deposit_spl_and_call_gateway_instr(
    config: &ClientConfig,
    mint: Pubkey,
    amount: u64,
    target_contract: [u8; 20],
    receiver: [u8; 20],
    payload: Vec<u8>,
    external_id: Option<[u8; 32]>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_program;

    // 如果external_id为None，则随机生成一个32字节的external_id
    let external_id = external_id.unwrap_or(rand::random());
    let mut payload = payload;
    payload.splice(0..0, external_id.to_vec());

    let ix_data = DepositSplAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload,
        revert_options: Some(RevertOptions {
            revert_address: config.gateway_send_program,
            abort_address: receiver,
            call_on_revert: true,
            revert_message: hex::decode("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000022000000000000000000000000000000000000000000000000000000000000000073c86a896f9ea09859efb5693feb4e9252d436ceb03946619b2031c43933078d9000000000000000000000000000000000000000000000000000000000000000118a14c1ff4fdcdb919aadb9fc2340cc5047960db89930154409cccdf9a65bb42000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace200000000000000000000000000000000000000000000000000000000000000014c822ac7de8177a902894755a08b7aeb053b08b4f039bf5eca89548c1c2cd80400000000000000000000000000000000000000000000000000000000000000017c4ef3fa70f65d4cea29805f8ea66e662f66b3c13ff2617f1d37b37105ac71ba0000000000000000000000000000000000000000000000000000000000000001e92839550965ffd4d64acaaf46d45df7318e5b4f57c90c487d60625d829b837b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040d2638ca121027536f80b69093e83eba68b3ff11f9253af514b29affbd7bfa1c5ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace2").unwrap(),
            on_revert_gas_limit: 10000000,
        }),
        deposit_fee: DEPOSIT_FEE,
    };

    let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);
    let (whitelisted_mint, _) =
        Pubkey::find_program_address(&[b"whitelist", mint.as_ref()], &program_id);
    let payer_token_account =
        spl_associated_token_account::get_associated_token_address(&payer.pubkey(), &mint);
    let program_token_account =
        spl_associated_token_account::get_associated_token_address(&gateway_meta, &mint);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(gateway_meta, false),
            AccountMeta::new(whitelisted_mint, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(anchor_spl::token::ID, false),
            AccountMeta::new(payer_token_account, false),
            AccountMeta::new(program_token_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = [14, 181, 27, 187, 171, 61, 237, 147].to_vec(); // deposit_spl_token_and_call
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

#[cfg(test)]
mod tests {
    use crate::EvmAddress;
    use crate::Pubkey;

    use super::*;
    use anchor_client::{
        anchor_lang::AnchorDeserialize,
        solana_client::rpc_client::RpcClient,
        solana_sdk::{message::Message, transaction::Transaction},
    };
    use anchor_spl::token;
    use base64::Engine;
    use gateway_send::instruction::OnRevert;
    use gateway_send::{
        gateway_send::{decode_bytes32, decode_bytes_with_length, decode_u16, decode_u256},
        states::events::EddyCrossChainReceive,
    };

    use std::str::FromStr;

    fn get_test_config() -> ClientConfig {
        ClientConfig {
            payer_path: "/Users/jwq/.config/solana/test_id.json".to_string(),
            gateway_send_program: Pubkey::from_str("CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK")
                .unwrap(),
            gateway_program: Pubkey::from_str("ZETAjseVjuFsxdRxo6MmTCvqFwb3ZHUx56Co3vCmGis")
                .unwrap(),
            http_url: "https://api.mainnet-beta.solana.com".to_string(),
            ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
            admin_path: "/Users/jwq/.config/solana/test_id.json".to_string(),
            sol_solana_zrc20: EvmAddress::from_str("0x4bC32034caCcc9B7e02536945eDbC286bACbA073")
                .unwrap(),
            usdc_solana_zrc20: EvmAddress::from_str("0x8344d6f84d26f998fa070BbEA6D2E15E359e2641")
                .unwrap(),
            gateway_transfer_native: EvmAddress::from_str(
                "0x63eEc8527884582358Ce6e93d530Df725D5Cf7d1",
            )
            .unwrap(),
        }
    }

    fn get_test_external_id() -> [u8; 32] {
        hex::decode("d2638ca121027536f80b69093e83eba68b3ff11f9253af514b29affbd7bfa1c5")
            .unwrap()
            .try_into()
            .unwrap()
    }

    #[test]
    fn test_deposit_sol_and_call_gateway_instr() {
        let config = get_test_config();
        let external_id = get_test_external_id();

        let amount = 1000000;
        let target_contract = config.gateway_transfer_native.0;
        let receiver = EvmAddress::from_str("0x4B37ff61e17DdcD4cEA80AF768de9455FC373764")
            .unwrap()
            .0;
        let mut payload = vec![];
        payload.extend_from_slice(&receiver);
        payload.extend_from_slice(&config.sol_solana_zrc20.0);

        let instructions = deposit_sol_and_call_gateway_instr(
            &config,
            amount,
            target_contract,
            receiver,
            payload.clone(),
            Some(external_id),
        )
        .unwrap();
        let transaction = Transaction::new_with_payer(
            &instructions,
            Some(&Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap()),
        );

        let data = transaction.message.serialize();
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);

        let expected_data = "AQACBK5dXT15CLloc2FYRbWMW/iUNxqGamtqateG1tBOdqziGKFMH/T9zbkZqtufwjQMxQR5YNuJkwFUQJzM35plu0IAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAhBy5DHwXY2oh7LR9Ulmj0f6s9aTLB7QPRVejXMW2vIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAwMAAQK+AUEhusZy34U5QEIPAAAAAADIhJIEnJkMDvLrD3fRrvjWa/FrqEgAAADSY4yhIQJ1NvgLaQk+g+umiz/xH5JTr1FLKa/717+hxUs3/2HhfdzUzqgK92jelFX8Nzdkrfc+uj66pyVOhZVJpEx073z/dQEBrl1dPXkIuWhzYVhFtYxb+JQ3GoZqa2pq14bW0E52rOIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAA=";
        assert_eq!(data_base64, expected_data);
    }

    #[test]
    fn test_deposit_spl_and_call_gateway_instr() {
        let config = get_test_config();
        let external_id = get_test_external_id();

        let amount = 1000000;
        let target_contract = config.gateway_transfer_native.0;
        let receiver = EvmAddress::from_str("0x4B37ff61e17DdcD4cEA80AF768de9455FC373764")
            .unwrap()
            .0;
        let mut payload = vec![];
        payload.extend_from_slice(&receiver);
        payload.extend_from_slice(
            &EvmAddress::from_str("0xD10932EB3616a937bd4a2652c87E9FeBbAce53e5")
                .unwrap()
                .0,
        );

        let instructions = deposit_spl_and_call_gateway_instr(
            &config,
            Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr").unwrap(),
            amount,
            target_contract,
            receiver,
            payload.clone(),
            Some(external_id),
        )
        .unwrap();

        // println!("instructions: {}", instructions[0].program_id);
        // for account in instructions[0].accounts.iter() {
        //     println!(
        //         "instructions: {}, {}, {}",
        //         account.pubkey, account.is_signer, account.is_writable
        //     );
        // }
        // println!(
        //     "instructions: {}",
        //     hex::encode(instructions[0].data.clone())
        // );

        let transaction = Transaction::new_with_payer(
            &instructions,
            Some(&Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap()),
        );

        let data = transaction.message.serialize();
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);

        let expected_data = "AQACCa5dXT15CLloc2FYRbWMW/iUNxqGamtqateG1tBOdqziBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkYoUwf9P3NuRmq25/CNAzFBHlg24mTAVRAnMzfmmW7QikD0EsV88ykamEEDF9rq2/uL/c1ouddWdWWQbb+YSVwfE7z+nD2XUzqKYBfjqZuZi9ms8E/8mF/HTezcQWscbq/VFTOKsKzyMfNo6Lcq7293LRH1THnBFjsipdRAXTRIukoOVUJZf/U1krKr0bUXfcxjltPV8kMSH1gYl2Cm4N7AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIQcuQx8F2NqIey0fVJZo9H+rPWkywe0D0VXo1zFtryAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQgIAAIDBgEEBQe+AQ61G7urPe2TQEIPAAAAAADIhJIEnJkMDvLrD3fRrvjWa/FrqEgAAADSY4yhIQJ1NvgLaQk+g+umiz/xH5JTr1FLKa/717+hxUs3/2HhfdzUzqgK92jelFX8Nzdk0Qky6zYWqTe9SiZSyH6f67rOU+UBrl1dPXkIuWhzYVhFtYxb+JQ3GoZqa2pq14bW0E52rOIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAA=";
        assert_eq!(data_base64, expected_data);
    }

    #[test]
    fn test_on_call_data() {
        let data = [
            36, 225, 20, 26, 174, 247, 111, 178, 250, 84, 146, 59, 78, 63, 127, 74, 132, 0, 167,
            121, 144, 107, 94, 102, 162, 125, 115, 188, 167, 148, 80, 44, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 205, 160, 0, 44, 0,
            0, 67, 106, 101, 87, 101, 103, 55, 80, 102, 121, 113, 53, 86, 99, 97, 107, 120, 97, 85,
            119, 66, 72, 67, 90, 111, 69, 101, 80, 75, 89, 117, 90, 84, 89, 103, 102, 107, 88, 97,
            97, 105, 67, 119, 51,
        ];

        let mut offset = 0;
        let external_id = decode_bytes32(&data, &mut offset);
        let output_amount = decode_u256(&data, &mut offset);
        let receiver_len = decode_u16(&data, &mut offset);
        let swap_data_len = decode_u16(&data, &mut offset);
        let receiver = decode_bytes_with_length(&data, &mut offset, receiver_len as usize);
        let receiver_str = String::from_utf8(receiver).unwrap();
        let swap_data = decode_bytes_with_length(&data, &mut offset, swap_data_len as usize);

        assert_eq!(
            hex::encode(external_id),
            "24e1141aaef76fb2fa54923b4e3f7f4a8400a779906b5e66a27d73bca794502c"
        );
        assert_eq!(output_amount, 8900000);
        assert_eq!(receiver_str, "CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3");
        assert_eq!(hex::encode(swap_data), "");
    }

    #[test]
    fn test_simulate_on_call() {
        let instruction = Instruction {
            program_id: Pubkey::from_str("CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK").unwrap(),
            accounts: vec![
                AccountMeta::new(
                    Pubkey::from_str("55GZAataCYtYidZDHmYigCKAxENi4YPfwT16wbT5iCgG").unwrap(),
                    false,
                ),
                AccountMeta::new(
                    Pubkey::from_str("2f9SLuUNb7TNeM6gzBwT4ZjbL5ZyKzzHg1Ce9yiquEjj").unwrap(),
                    false,
                ),
                AccountMeta::new_readonly(token::ID, false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new(
                    Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap(),
                    true,
                ),
                // AccountMeta::new(
                //     Pubkey::from_str("69f77rA4acX8U13rQyPkpCGD6QXMCRnpFzbChxKLtiqy").unwrap(),
                //     false,
                // ),
                // AccountMeta::new(
                //     Pubkey::from_str("9NFP6ezMNXAkvfGFojqgMiMoZiCCMYGEQAQsMfKLv7aq").unwrap(),
                //     false,
                // ),
                // AccountMeta::new_readonly(
                //     Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr").unwrap(),
                //     false,
                // ),
            ],
            data: {
                hex::decode("10884220fe28b508a0cd870000000000351a86a2c8dc47d396305aacd7f126e096b2eee47000000086879cef4a0c4ee478b07ee1df616fd9b91342203d946c6b83f6e40c20a2d737000000000000000000000000000000000000000000000000000000000087cda0002c0000436a655765673750667971355663616b786155774248435a6f4565504b59755a545967666b58616169437733").unwrap()
            },
        };

        let wallet = read_keypair_file("/Users/jwq/.config/solana/test_id.json").unwrap();
        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap()),
            &[&wallet],
            recent_blockhash,
        );
        let simulation = rpc_client.simulate_transaction(&transaction).unwrap();
        if simulation.value.err.is_some() {
            println!("simulation: {:?}", simulation.value.logs);
        }
        assert!(simulation.value.err.is_none());
    }

    #[test]
    fn test_simulate_on_revert() {
        let instruction = Instruction {
            program_id: Pubkey::from_str("CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK").unwrap(),
            accounts: vec![
                AccountMeta::new(
                    Pubkey::from_str("55GZAataCYtYidZDHmYigCKAxENi4YPfwT16wbT5iCgG").unwrap(),
                    false,
                ),
                AccountMeta::new(
                    Pubkey::from_str("2f9SLuUNb7TNeM6gzBwT4ZjbL5ZyKzzHg1Ce9yiquEjj").unwrap(),
                    false,
                ),
                AccountMeta::new_readonly(token::ID, false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new(
                    Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap(),
                    true,
                ),
                // AccountMeta::new(
                //     Pubkey::from_str("69f77rA4acX8U13rQyPkpCGD6QXMCRnpFzbChxKLtiqy").unwrap(),
                //     false,
                // ),
                // AccountMeta::new(
                //     Pubkey::from_str("9NFP6ezMNXAkvfGFojqgMiMoZiCCMYGEQAQsMfKLv7aq").unwrap(),
                //     false,
                // ),
                // AccountMeta::new_readonly(
                //     Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr").unwrap(),
                //     false,
                // ),
            ],
            data: {
                let mut data = vec![226, 44, 101, 52, 224, 214, 41, 9];
                let ix_data = OnRevert {
                    amount: 1000000,
                    sender: Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3")
                        .unwrap(),
                    data: hex::decode("d2638ca121027536f80b69093e83eba68b3ff11f9253af514b29affbd7bfa1c5ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace2").unwrap(),
                };
                data.extend(ix_data.try_to_vec().unwrap());
                data
            },
        };

        let wallet = read_keypair_file("/Users/jwq/.config/solana/test_id.json").unwrap();
        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap()),
            &[&wallet],
            recent_blockhash,
        );
        let simulation = rpc_client.simulate_transaction(&transaction).unwrap();
        if simulation.value.err.is_some() {
            println!("simulation: {:?}", simulation.value.logs);
        }
        assert!(simulation.value.err.is_none());
    }

    #[test]
    fn test_simulate_from_base64() {
        let base64_tx = "AQACBK5dXT15CLloc2FYRbWMW/iUNxqGamtqateG1tBOdqziGKFMH/T9zbkZqtufwjQMxQR5YNuJkwFUQJzM35plu0IAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAhBy5DHwXY2oh7LR9Ulmj0f6s9aTLB7QPRVejXMW2vIRZqHeyXdbjVaMtgPehB0OL2dOUHcEYiedzRMrmzlUUUBAwMAAQK+AUEhusZy34U5QEIPAAAAAAA1GoaiyNxH05YwWqzX8SbglrLu5EgAAAABI16FfjEPsWPf46jjHqXao/9yVWno3WWeQnALuJ7p40s3/2HhfdzUzqgK92jelFX8Nzdkrfc+uj66pyVOhZVJpEx073z/dQEBrl1dPXkIuWhzYVhFtYxb+JQ3GoZqa2pq14bW0E52rOIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAA=";
        let tx_data = base64::engine::general_purpose::STANDARD
            .decode(base64_tx)
            .unwrap();

        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

        let mut message: Message = bincode::deserialize(&tx_data).unwrap();
        message.recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new_unsigned(message);

        let simulation = rpc_client.simulate_transaction(&transaction).unwrap();

        // 打印结果
        if simulation.value.err.is_some() {
            println!("Simulation error: {:?}", simulation.value.err);
            println!("Simulation logs: {:?}", simulation.value.logs);
        } else {
            println!("Simulation successful");
            println!("Simulation logs: {:?}", simulation.value.logs);
        }

        // 验证结果
        assert!(simulation.value.err.is_none());
    }

    #[test]
    fn test_event_decode() {
        let event = "mMRaE/l8oiQl7BqWR/ck1Ido+cZ9J7ng4vtppQDfbZfUtdhAwmitvAabiFf+q4GE+2h/Y0YYwDXaxDncGus7VZig8AAAAAAABpuIV/6rgYT7aH9jRhjANdrEOdwa6ztVmKDwAAAAAADaLm6yAAAAANoubrIAAAAAH9keflJ63/Q6e9s9yQhGHLwcHLwfrL0L51pOTNuOUa4AAAAA";
        let buf = base64::engine::general_purpose::STANDARD
            .decode(event)
            .unwrap();
        let event = EddyCrossChainReceive::try_from_slice(&buf[8..]).unwrap();
        assert_eq!(
            "39KfGpqjWdnrGkTXahLoBMiMjHP5U3qpvjqA37359qWV",
            event.wallet_address.to_string()
        );
    }

    #[test]
    fn test_sol() {
        let sol = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        println!("sol: {}", sol);
        println!("default: {}", Pubkey::default());
        assert_eq!(sol, Pubkey::default());
    }

    #[test]
    fn test_on_revert_data() {
        let external_id = get_test_external_id();
        let sender = Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap();
        let mut data = external_id.to_vec();
        data.extend(sender.to_bytes());
        println!("data: {}", hex::encode(data));
    }

    #[test]
    fn test_encode_on_revert_call() {
        let config_pda = Pubkey::from_str("55GZAataCYtYidZDHmYigCKAxENi4YPfwT16wbT5iCgG").unwrap();
        let gateway_pda = Pubkey::from_str("2f9SLuUNb7TNeM6gzBwT4ZjbL5ZyKzzHg1Ce9yiquEjj").unwrap();
        let token_program = anchor_spl::token::ID;
        let system_program = system_program::id();
        let amount = 1000000;
        let sender = Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap();
        let external_id = get_test_external_id();

        // Test SOL transfer (1 remaining account)
        let recipient = Pubkey::from_str("9NFP6ezMNXAkvfGFojqgMiMoZiCCMYGEQAQsMfKLv7aq").unwrap();
        let remaining_accounts_sol = vec![recipient];

        let encoded_sol = encode_on_revert_call(
            &config_pda,
            &gateway_pda,
            &token_program,
            &system_program,
            amount,
            &sender,
            external_id,
            &remaining_accounts_sol,
        );

        // Decode and verify
        let (accounts, decoded_amount, decoded_sender, decoded_external_id) =
            decode_on_revert_call(&encoded_sol).unwrap();

        assert_eq!(decoded_amount, amount);
        assert_eq!(decoded_sender, sender);
        assert_eq!(decoded_external_id, external_id);
        assert_eq!(accounts.len(), 5); // 4 base accounts + 1 recipient

        // Verify the recipient account is writable
        assert_eq!(accounts[4].0, recipient);
        assert_eq!(accounts[4].1, true); // should be writable

        println!("SOL transfer on_revert encoding test passed");

        // Test token transfer (4 remaining accounts)
        let from_account =
            Pubkey::from_str("69f77rA4acX8U13rQyPkpCGD6QXMCRnpFzbChxKLtiqy").unwrap();
        let to_account = Pubkey::from_str("9NFP6ezMNXAkvfGFojqgMiMoZiCCMYGEQAQsMfKLv7aq").unwrap();
        let mint = Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr").unwrap();
        let some_account = Pubkey::from_str("11111111111111111111111111111111").unwrap();

        let remaining_accounts_token = vec![some_account, from_account, to_account, mint];

        let encoded_token = encode_on_revert_call(
            &config_pda,
            &gateway_pda,
            &token_program,
            &system_program,
            amount,
            &sender,
            external_id,
            &remaining_accounts_token,
        );

        // Decode and verify token transfer
        let (accounts_token, decoded_amount_token, decoded_sender_token, decoded_external_id_token) =
            decode_on_revert_call(&encoded_token).unwrap();

        assert_eq!(decoded_amount_token, amount);
        assert_eq!(decoded_sender_token, sender);
        assert_eq!(decoded_external_id_token, external_id);
        assert_eq!(accounts_token.len(), 8); // 4 base accounts + 4 token accounts

        // Verify token accounts have correct writable flags
        assert_eq!(accounts_token[4].0, some_account);
        assert_eq!(accounts_token[4].1, false); // not writable
        assert_eq!(accounts_token[5].0, from_account);
        assert_eq!(accounts_token[5].1, true); // writable
        assert_eq!(accounts_token[6].0, to_account);
        assert_eq!(accounts_token[6].1, true); // writable
        assert_eq!(accounts_token[7].0, mint);
        assert_eq!(accounts_token[7].1, false); // not writable

        println!("Token transfer on_revert encoding test passed");
    }

    #[test]
    fn test_encode_native_message() {
        // Test data with real addresses
        let target_zrc20: [u8; 20] = hex::decode("4bC32034caCcc9B7e02536945eDbC286bACbA073")
            .unwrap()
            .try_into()
            .unwrap();
        let sender = hex::decode("4B37ff61e17DdcD4cEA80AF768de9455FC373764").unwrap();
        let receiver = hex::decode("D10932EB3616a937bd4a2652c87E9FeBbAce53e5").unwrap();
        let swap_data = hex::decode("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000000043c86a896f9ea09859efb5693feb4e9252d436ceb03946619b2031c43933078d9000000000000000000000000000000000000000000000000000000000000000118a14c1ff4fdcdb919aadb9fc2340cc5047960db89930154409cccdf9a65bb42000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040d2638ca121027536f80b69093e83eba68b3ff11f9253af514b29affbd7bfa1c5ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace2")
            .unwrap();

        // Encode
        let encoded = encode_native_message(&target_zrc20, &sender, &receiver, &swap_data);

        // Verify structure: 20 + 2 + 2 + 20 + 20 + 352 = 416 bytes
        assert_eq!(
            encoded.len(),
            20 + 2 + 2 + sender.len() + receiver.len() + swap_data.len()
        );

        // Decode and verify
        let (decoded_target, decoded_sender, decoded_receiver, decoded_swap_data) =
            decode_native_message(&encoded).unwrap();

        assert_eq!(decoded_target, target_zrc20);
        assert_eq!(decoded_sender, sender);
        assert_eq!(decoded_receiver, receiver);
        assert_eq!(decoded_swap_data, swap_data);

        println!("Native message encoding test passed");
        println!("Target ZRC20: 0x{}", hex::encode(target_zrc20));
        println!("Sender: 0x{}", hex::encode(&sender));
        println!("Receiver: 0x{}", hex::encode(&receiver));
        println!("Swap data length: {} bytes", swap_data.len());
        println!("Encoded total length: {} bytes", encoded.len());

        // Test with empty data
        let empty_sender = b"".to_vec();
        let empty_receiver = b"".to_vec();
        let empty_swap_data = b"".to_vec();

        let encoded_empty = encode_native_message(
            &target_zrc20,
            &empty_sender,
            &empty_receiver,
            &empty_swap_data,
        );
        let (
            decoded_target_empty,
            decoded_sender_empty,
            decoded_receiver_empty,
            decoded_swap_data_empty,
        ) = decode_native_message(&encoded_empty).unwrap();

        assert_eq!(decoded_target_empty, target_zrc20);
        assert_eq!(decoded_sender_empty, empty_sender);
        assert_eq!(decoded_receiver_empty, empty_receiver);
        assert_eq!(decoded_swap_data_empty, empty_swap_data);

        println!("Native message encoding with empty data test passed");
    }

    #[test]
    fn test_decode_native_message_edge_cases() {
        // Test with insufficient data
        let insufficient_data = vec![0u8; 23]; // Less than minimum 24 bytes
        let result = decode_native_message(&insufficient_data);
        assert!(result.is_err());

        // Test with valid minimum data
        let mut valid_data = vec![0u8; 24];
        valid_data[20] = 0; // sender length = 0
        valid_data[21] = 0;
        valid_data[22] = 0; // receiver length = 0
        valid_data[23] = 0;

        let result = decode_native_message(&valid_data);
        assert!(result.is_ok());

        let (target, sender, receiver, swap_data) = result.unwrap();
        assert_eq!(target, [0u8; 20]);
        assert_eq!(sender, Vec::<u8>::new());
        assert_eq!(receiver, Vec::<u8>::new());
        assert_eq!(swap_data, Vec::<u8>::new());

        println!("Native message decode edge cases test passed");
    }
}

/// Encode native message similar to Solidity's encodeNativeMessage function
/// This function encodes: bytes20(targetZRC20) + uint16(sender.length) + uint16(receiver.length) + sender + receiver + swapData
pub fn encode_native_message(
    target_zrc20: &[u8; 20],
    sender: &[u8],
    receiver: &[u8],
    swap_data: &[u8],
) -> Vec<u8> {
    let mut encoded = Vec::new();
    // Add targetZRC20 (20 bytes)
    encoded.extend_from_slice(target_zrc20);
    // Add sender length (2 bytes, big-endian)
    let sender_len = sender.len() as u16;
    encoded.extend_from_slice(&sender_len.to_be_bytes());
    // Add receiver length (2 bytes, big-endian)
    let receiver_len = receiver.len() as u16;
    encoded.extend_from_slice(&receiver_len.to_be_bytes());
    // Add sender data
    encoded.extend_from_slice(sender);
    // Add receiver data
    encoded.extend_from_slice(receiver);
    // Add swap data
    encoded.extend_from_slice(swap_data);
    encoded
}

/// Decode native message
pub fn decode_native_message(
    encoded_data: &[u8],
) -> anyhow::Result<([u8; 20], Vec<u8>, Vec<u8>, Vec<u8>)> {
    if encoded_data.len() < 24 {
        anyhow::bail!("Invalid encoded_data length");
    }
    let mut offset = 0;
    // Decode targetZRC20 (20 bytes)
    let target_zrc20: [u8; 20] = encoded_data[offset..offset + 20].try_into().unwrap();
    offset += 20;
    // Decode sender length (2 bytes, big-endian)
    let sender_len = u16::from_be_bytes([encoded_data[offset], encoded_data[offset + 1]]) as usize;
    offset += 2;
    // Decode receiver length (2 bytes, big-endian)
    let receiver_len =
        u16::from_be_bytes([encoded_data[offset], encoded_data[offset + 1]]) as usize;
    offset += 2;
    // Check if we have enough data
    if offset + sender_len + receiver_len > encoded_data.len() {
        anyhow::bail!("Invalid encoded_data length");
    }
    // Decode sender data
    let sender = encoded_data[offset..offset + sender_len].to_vec();
    offset += sender_len;
    // Decode receiver data
    let receiver = encoded_data[offset..offset + receiver_len].to_vec();
    offset += receiver_len;
    // Decode swap data (remaining bytes)
    let swap_data = encoded_data[offset..].to_vec();
    Ok((target_zrc20, sender, receiver, swap_data))
}
