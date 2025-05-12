use anchor_client::{
    anchor_lang::{prelude::AccountMeta, AnchorSerialize, Discriminator},
    solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program},
};
use anchor_spl::associated_token::spl_associated_token_account;
use anyhow::Result;
use gateway_send::{
    gateway_send::{DepositAndCallArgs, DepositArgs, DepositSplAndCallArgs, RevertOptions},
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

    let ix_data = DepositArgs {
        amount,
        receiver,
        revert_options: Some(RevertOptions {
            revert_address: payer.pubkey(),
            abort_address: Pubkey::default(),
            call_on_revert: true,
            revert_message: Vec::new(),
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
            revert_address: payer.pubkey(),
            abort_address: Pubkey::default(),
            call_on_revert: true,
            revert_message: Vec::new(),
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

    use super::*;
    use anchor_client::solana_sdk::transaction::Transaction;
    use base64::Engine;
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
        let mut payload = vec![];
        payload.extend_from_slice(&external_id);
        payload.extend_from_slice(
            &EvmAddress::from_str("0x4B37ff61e17DdcD4cEA80AF768de9455FC373764")
                .unwrap()
                .0,
        );
        payload.extend_from_slice(&config.sol_solana_zrc20.0);

        let instructions = deposit_sol_and_call_gateway_instr(
            &config,
            amount,
            target_contract,
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

        let expected_data = "AQACBK5dXT15CLloc2FYRbWMW/iUNxqGamtqateG1tBOdqziGKFMH/T9zbkZqtufwjQMxQR5YNuJkwFUQJzM35plu0IAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAhBy5DHwXY2oh7LR9Ulmj0f6s9aTLB7QPRVejXMW2vIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAwMAAQLeAUEhusZy34U5QEIPAAAAAADIhJIEnJkMDvLrD3fRrvjWa/FrqGgAAADSY4yhIQJ1NvgLaQk+g+umiz/xH5JTr1FLKa/717+hxdJjjKEhAnU2+AtpCT6D66aLP/EfklOvUUspr/vXv6HFSzf/YeF93NTOqAr3aN6UVfw3N2St9z66PrqnJU6FlUmkTHTvfP91AQGuXV09eQi5aHNhWEW1jFv4lDcahmpramrXhtbQTnas4gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAA==";
        assert_eq!(data_base64, expected_data);
    }

    #[test]
    fn test_deposit_spl_and_call_gateway_instr() {
        let config = get_test_config();
        let external_id = get_test_external_id();

        let amount = 1000000;
        let target_contract = config.gateway_transfer_native.0;
        let mut payload = vec![];
        payload.extend_from_slice(&external_id);
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
        let transaction = Transaction::new_with_payer(
            &instructions,
            Some(&Pubkey::from_str("CjeWeg7Pfyq5VcakxaUwBHCZoEePKYuZTYgfkXaaiCw3").unwrap()),
        );

        let data = transaction.message.serialize();
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);

        let expected_data = "AQACCa5dXT15CLloc2FYRbWMW/iUNxqGamtqateG1tBOdqziBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkYoUwf9P3NuRmq25/CNAzFBHlg24mTAVRAnMzfmmW7QikD0EsV88ykamEEDF9rq2/uL/c1ouddWdWWQbb+YSVwfE7z+nD2XUzqKYBfjqZuZi9ms8E/8mF/HTezcQWscbq/VFTOKsKzyMfNo6Lcq7293LRH1THnBFjsipdRAXTRIukoOVUJZf/U1krKr0bUXfcxjltPV8kMSH1gYl2Cm4N7AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIQcuQx8F2NqIey0fVJZo9H+rPWkywe0D0VXo1zFtryAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQgIAAIDBgEEBQfeAQ61G7urPe2TQEIPAAAAAADIhJIEnJkMDvLrD3fRrvjWa/FrqGgAAADSY4yhIQJ1NvgLaQk+g+umiz/xH5JTr1FLKa/717+hxdJjjKEhAnU2+AtpCT6D66aLP/EfklOvUUspr/vXv6HFSzf/YeF93NTOqAr3aN6UVfw3N2TRCTLrNhapN71KJlLIfp/rus5T5QGuXV09eQi5aHNhWEW1jFv4lDcahmpramrXhtbQTnas4gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAA==";
        assert_eq!(data_base64, expected_data);
    }
}
