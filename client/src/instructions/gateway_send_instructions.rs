use anchor_client::{
    anchor_lang::{prelude::AccountMeta, AnchorSerialize, Discriminator},
    solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
        system_program,
    },
};
use anchor_spl::associated_token::spl_associated_token_account;
use anyhow::Result;
use gateway_send::{
    gateway_send::{
        calc_external_id, DepositAndCallArgs, DepositArgs, DepositSplAndCallArgs, RevertOptions,
    },
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
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_program;

    // 随机生成一个32字节的external_id
    let external_id: [u8; 32] = rand::random();
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
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let program_id = config.gateway_program;

    // 随机生成一个32字节的external_id
    let external_id: [u8; 32] = rand::random();
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
