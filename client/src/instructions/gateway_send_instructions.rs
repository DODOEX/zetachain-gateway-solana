use anchor_client::{
    anchor_lang::{prelude::AccountMeta, AnchorSerialize, Discriminator},
    solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
        system_program,
    },
};
use anyhow::Result;
use gateway_send::{AUTHORITY_SEED, CONFIG_SEED};

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

pub fn deposit_sol_and_call_instr(
    config: &ClientConfig,
    amount: u64,
    target_contract: [u8; 20],
    payload: Vec<u8>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let (program_authority, _) = Pubkey::find_program_address(&[AUTHORITY_SEED], &program_id);

    let ix_data = gateway_send::instruction::DepositSolAndCall {
        amount,
        target_contract,
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

pub fn deposit_and_call_instr(
    config: &ClientConfig,
    amount: u64,
    target_contract: Pubkey,
    payload: Vec<u8>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::DepositAndCall {
        amount,
        target_contract,
        payload,
    };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::DepositAndCall::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

pub fn deposit_swap_and_call_instr(
    config: &ClientConfig,
    amount: u64,
    swap_data: Vec<u8>,
    target_contract: Pubkey,
    asset: Pubkey,
    payload: Vec<u8>,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_send_program;

    let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
    let ix_data = gateway_send::instruction::DepositSwapAndCall {
        amount,
        swap_data,
        target_contract,
        asset,
        payload,
    };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: {
            let mut data = gateway_send::instruction::DepositSwapAndCall::DISCRIMINATOR.to_vec();
            data.extend(ix_data.try_to_vec().unwrap());
            data
        },
    };
    Ok(vec![instruction])
}

// pub fn send_instr(
//     config: &ClientConfig,
//     token_mint: Pubkey,
//     token_account: Pubkey,
//     dst_chain: u32,
//     unique_message_account_keypair: &Keypair,
// ) -> Result<Vec<Instruction>> {
//     let payer = read_keypair_file(&config.payer_path)?;
//     let program_id = config.gateway_send_program;

//     let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
//     let (other_chain_tokens_pda, _) = Pubkey::find_program_address(
//         &[
//             b"other_chain_tokens",
//             &dst_chain.to_le_bytes(),
//             token_mint.as_ref(),
//         ],
//         &program_id,
//     );
//     let (token_allowed_pda, _) =
//         Pubkey::find_program_address(&[b"tokens", token_mint.as_ref()], &program_id);
//     let (program_authority_pda, _) =
//         Pubkey::find_program_address(&[PROGRAM_AUTHORITY_SEED], &program_id);
//     // 计算ata地址
//     let program_token_account = get_associated_token_address(&program_authority_pda, &token_mint);

//     let (storage_pda, _) = Pubkey::find_program_address(&[HYPERLANE_STORAGE_SEED], &program_id);
//     let (outbox_account, _outbox_bump) =
//         Pubkey::find_program_address(&[b"hyperlane", b"-", b"outbox"], &config.hyperlane_mailbox);
//     let (dispatch_authority_key, _dispatch_authority_bump) = Pubkey::find_program_address(
//         &[b"hyperlane_dispatcher", b"-", b"dispatch_authority"],
//         &program_id,
//     );
//     let (dispatched_message_account_key, _dispatched_message_bump) = Pubkey::find_program_address(
//         &[
//             b"hyperlane",
//             b"-",
//             b"dispatched_message",
//             b"-",
//             (unique_message_account_keypair.pubkey().as_ref()),
//         ],
//         &config.hyperlane_mailbox,
//     );
//     let (igp_program_data_account_key, _igp_program_data_bump) = Pubkey::find_program_address(
//         &[b"hyperlane_igp", b"-", b"program_data"],
//         &config.hyperlane_igp,
//     );
//     let (gas_payment_key, _) = Pubkey::find_program_address(
//         &[
//             b"hyperlane_igp",
//             b"-",
//             b"gas_payment",
//             b"-",
//             unique_message_account_keypair.pubkey().as_ref(),
//         ],
//         &config.hyperlane_igp,
//     );

//     // 使用 Anchor 生成 discriminator
//     let ix_data = gateway_send::instruction::Send {
//         dst_chain: 53456,
//         receiver: H256::from_hex("0x4b37ff61e17ddcd4cea80af768de9455fc373764").unwrap(),
//         amount: 1000000,
//         min_receive_amount: 1000000,
//         // target: H256::from_hex("0x3af0a7d5a4fde890b662f6af6e7ad05ead2ebfa5").unwrap(),
//         composer: H256::default(),
//         data: vec![],
//     };
//     let instruction = Instruction {
//         program_id,
//         data: {
//             let mut data = gateway_send::instruction::Send::DISCRIMINATOR.to_vec();
//             data.extend(ix_data.try_to_vec().unwrap());
//             data
//         },
//         accounts: vec![
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new(token_mint, false),
//             AccountMeta::new(token_account, false),
//             AccountMeta::new(config_pda, false),
//             AccountMeta::new(other_chain_tokens_pda, false),
//             AccountMeta::new(token_allowed_pda, false),
//             AccountMeta::new(program_authority_pda, false),
//             AccountMeta::new(program_token_account, false),
//             AccountMeta::new(storage_pda, false),
//             AccountMeta::new(dispatch_authority_key, false),
//             AccountMeta::new_readonly(token::ID, false),
//             AccountMeta::new_readonly(associated_token::ID, false),
//             AccountMeta::new_readonly(system_program::id(), false),
//             // remaining accounts
//             AccountMeta::new(outbox_account, false),
//             AccountMeta::new(unique_message_account_keypair.pubkey(), true),
//             AccountMeta::new(dispatched_message_account_key, false),
//             AccountMeta::new(igp_program_data_account_key, false),
//             AccountMeta::new(gas_payment_key, false),
//             AccountMeta::new_readonly(config.hyperlane_igp, false),
//             AccountMeta::new_readonly(spl_noop::id(), false),
//             AccountMeta::new_readonly(config.hyperlane_mailbox, false),
//         ],
//     };

//     Ok(vec![instruction])
// }
