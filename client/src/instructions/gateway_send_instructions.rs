use anchor_client::{
    anchor_lang::{prelude::AccountMeta, AnchorSerialize, Discriminator},
    solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program},
};
use anchor_spl::associated_token::spl_associated_token_account;
use anyhow::Result;
use gateway_send::{
    gateway_send::{DepositAndCallArgs, DepositArgs, DepositSplAndCallArgs, RevertOptions},
    CONFIG_SEED,
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

// pub fn deposit_sol_and_call_instr(
//     config: &ClientConfig,
//     target_contract: [u8; 20],
//     amount: u64,
//     dst_chain_id: u32,
//     payload: Vec<u8>,
// ) -> Result<Vec<Instruction>> {
//     let payer = read_keypair_file(&config.payer_path)?;
//     let program_id = config.gateway_send_program;

//     let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
//     let (program_authority, _) = Pubkey::find_program_address(&[AUTHORITY_SEED], &program_id);

//     let ix_data = gateway_send::instruction::DepositSolAndCall {
//         target_contract,
//         amount,
//         dst_chain_id,
//         payload,
//     };

//     let (gateway_meta, _) = Pubkey::find_program_address(&[b"meta"], &config.gateway_program);

//     let instruction = Instruction {
//         program_id,
//         accounts: vec![
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new(config_pda, false),
//             AccountMeta::new(program_authority, false),
//             AccountMeta::new(config.gateway_program, false),
//             AccountMeta::new_readonly(system_program::id(), false),
//             // remaining accounts, gateway deposit with call accounts
//             AccountMeta::new(program_authority, false),
//             AccountMeta::new(gateway_meta, false),
//             AccountMeta::new_readonly(system_program::id(), false),
//         ],
//         data: {
//             let mut data = gateway_send::instruction::DepositSolAndCall::DISCRIMINATOR.to_vec();
//             data.extend(ix_data.try_to_vec().unwrap());
//             data
//         },
//     };
//     Ok(vec![instruction])
// }

// pub fn deposit_and_call_instr(
//     config: &ClientConfig,
//     amount: u64,
//     target_contract: Pubkey,
//     payload: Vec<u8>,
// ) -> Result<Vec<Instruction>> {
//     let payer = read_keypair_file(&config.payer_path)?;
//     let program_id = config.gateway_send_program;

//     let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
//     let ix_data = gateway_send::instruction::DepositAndCall {
//         amount,
//         target_contract,
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

    let mut abort_address = vec![0u8; 12];
    abort_address[12..].copy_from_slice(&receiver);
    let abort_address = Pubkey::new_from_array(abort_address.try_into().unwrap());
    let ix_data = DepositArgs {
        amount,
        receiver,
        revert_options: Some(RevertOptions {
            revert_address: config.gateway_send_program,
            abort_address,
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

    let mut abort_address = receiver.to_vec();
    abort_address.extend_from_slice(&vec![0u8; 12]);
    let abort_address = Pubkey::new_from_array(abort_address.try_into().unwrap());
    let ix_data = DepositAndCallArgs {
        amount,
        receiver: target_contract,
        message: payload,
        revert_options: Some(RevertOptions {
            revert_address: config.gateway_send_program,
            abort_address,
            call_on_revert: true,
            revert_message: hex::decode("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000000043c86a896f9ea09859efb5693feb4e9252d436ceb03946619b2031c43933078d9000000000000000000000000000000000000000000000000000000000000000118a14c1ff4fdcdb919aadb9fc2340cc5047960db89930154409cccdf9a65bb42000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ae5d5d3d7908b96873615845b58c5bf894371a866a6b6a6ad786d6d04e76ace20000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000b68656c6c6f20776f726c64000000000000000000000000000000000000000000").unwrap(),
            on_revert_gas_limit: 10000000,
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
            revert_address: payer.pubkey(),
            abort_address: Pubkey::default(),
            call_on_revert: true,
            revert_message: Vec::new(),
            on_revert_gas_limit: 0,
        }),
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
            http_url: "https://api.devnet.solana.com".to_string(),
            ws_url: "wss://api.devnet.solana.com/".to_string(),
            admin_path: "/Users/jwq/.config/solana/test_id.json".to_string(),
            sol_solana_zrc20: EvmAddress::from_str("0xADF73ebA3Ebaa7254E859549A44c74eF7cff7501")
                .unwrap(),
            gateway_transfer_native: EvmAddress::from_str(
                "0xc88492049C990c0eF2eB0F77D1Aef8D66Bf16ba8",
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
        let mut payload = vec![];
        payload.extend_from_slice(
            &EvmAddress::from_str("0x4B37ff61e17DdcD4cEA80AF768de9455FC373764")
                .unwrap()
                .0,
        );
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
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
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
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new(
                    Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap(),
                    true,
                ),
            ],
            data: {
                let mut data = vec![226, 44, 101, 52, 224, 214, 41, 9];
                let ix_data = OnRevert {
                    amount: 10000000,
                    sender: [0u8; 20],
                    data: hex::decode("68656c6c6f20776f726c64").unwrap(), // hello world
                };
                data.extend(ix_data.try_to_vec().unwrap());
                data
            },
        };

        let wallet = read_keypair_file("/Users/jwq/.config/solana/test_id.json").unwrap();
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
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

        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

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
}
