mod instructions;

use crate::instructions::gateway_send_instructions::{
    create_config_instr, update_dodo_route_proxy_instr, update_gateway_instr, update_owner_instr,
};
use gateway_send::{states::config::Config, CONFIG_SEED};

use std::{rc::Rc, str::FromStr};

use anchor_client::{
    anchor_lang::{prelude::Pubkey, AccountDeserialize},
    solana_client::rpc_client::RpcClient,
    solana_sdk::{self, signature::Keypair, signer::Signer, transaction::Transaction},
    Client, Cluster,
};
use anchor_spl::{
    associated_token::spl_associated_token_account,
    token::spl_token::{self},
    token_2022::spl_token_2022,
};
use anyhow::{format_err, Result};
use clap::Parser;
use configparser::ini::Ini;
use instructions::{
    gateway_send_instructions::{
        close_config_instr, deposit_sol_and_call_gateway_instr, deposit_sol_gateway_instr,
        deposit_spl_and_call_gateway_instr, update_gas_limit_instr,
    },
    lookup_table_instructions::{
        create_lookup_table_instr, deserialize_lookup_table, extend_lookup_table_instr,
    },
    token_instructions::{
        create_and_init_auxiliary_token, create_and_init_mint_instr,
        create_ata_token_account_instr, spl_token_mint_to_instr,
    },
};
use rand::rngs::OsRng;
use spl_token_client::{spl_token_2022::state::AccountState, token::ExtensionInitializationParams};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    http_url: String,
    ws_url: String,
    payer_path: String,
    admin_path: String,
    gateway_send_program: Pubkey,
    gateway_program: Pubkey,
    sol_solana_zrc20: EvmAddress,
    gateway_transfer_native: EvmAddress,
}

fn main() -> Result<()> {
    println!("Starting...");
    let client_config = "../client_config.ini";
    let client_config = load_cfg(&client_config.to_string()).unwrap();
    // Admin and cluster params.
    let payer = read_keypair_file(&client_config.payer_path)?;
    // let admin = read_keypair_file(&client_config.admin_path)?;
    // solana rpc client
    let rpc_client = RpcClient::new(client_config.http_url.to_string());

    // anchor client.
    let anchor_config = client_config.clone();
    let url = Cluster::Custom(anchor_config.http_url, anchor_config.ws_url);
    let wallet = read_keypair_file(&client_config.payer_path)?;
    let anchor_client = Client::new(url, Rc::new(wallet));
    let program = anchor_client.program(client_config.gateway_send_program)?;

    let opts = Opts::parse();
    match opts.command {
        CommandsName::CreateConfig {
            gateway,
            dodo_route_proxy,
        } => {
            let ix = create_config_instr(&client_config, gateway, dodo_route_proxy)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::CheckConfig {} => {
            let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program.id());
            println!("pda {}", config_pda);
            let account = rpc_client.get_account(&config_pda)?;
            let mut data = account.data.as_slice();
            let config: Config = AccountDeserialize::try_deserialize(&mut data)?;
            println!("{:?}", config);
        }
        CommandsName::UpdateGateway { gateway } => {
            let ix = update_gateway_instr(&client_config, gateway)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::UpdateDodoRouteProxy { dodo_route_proxy } => {
            let ix = update_dodo_route_proxy_instr(&client_config, dodo_route_proxy)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::UpdateGasLimit { new_gas_limit } => {
            let ix = update_gas_limit_instr(&client_config, new_gas_limit)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::UpdateOwner { new_owner } => {
            let ix = update_owner_instr(&client_config, new_owner)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::CloseConfig {} => {
            let ix = close_config_instr(&client_config)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }

        // CommandsName::DepositSolAndCall { amount, receiver } => {
        //     let target_contract = &client_config.gateway_transfer_native;
        //     let zrc20 = &client_config.sol_solana_zrc20;
        //     let mut payload = Vec::new();
        //     payload.extend_from_slice(&receiver.0);
        //     payload.extend_from_slice(&zrc20.0);
        //     let ix =
        //         deposit_sol_and_call_instr(&client_config, amount, target_contract.0, payload)?;
        //     let recent_blockhash = rpc_client.get_latest_blockhash()?;
        //     let transaction = Transaction::new_signed_with_payer(
        //         &ix,
        //         Some(&payer.pubkey()),
        //         &[&payer],
        //         recent_blockhash,
        //     );
        //     let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
        //         Ok(sig) => sig,
        //         Err(err) => {
        //             println!("Error: {:?}", err);
        //             return Err(err.into());
        //         }
        //     };
        //     println!("Signature: {:?}", signature);
        // }
        // CommandsName::DepositAndCall {
        //     amount,
        //     target_contract,
        //     payload,
        // } => {
        //     let ix = deposit_and_call_instr(&client_config, amount, target_contract, payload)?;
        //     let recent_blockhash = rpc_client.get_latest_blockhash()?;
        //     let transaction = Transaction::new_signed_with_payer(
        //         &ix,
        //         Some(&payer.pubkey()),
        //         &[&payer],
        //         recent_blockhash,
        //     );
        //     let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
        //         Ok(sig) => sig,
        //         Err(err) => {
        //             println!("Error: {:?}", err);
        //             return Err(err.into());
        //         }
        //     };
        //     println!("Signature: {:?}", signature);
        // }
        // CommandsName::DepositSwapAndCall {
        //     amount,
        //     swap_data,
        //     target_contract,
        //     asset,
        //     payload,
        // } => {
        //     let ix = deposit_swap_and_call_instr(
        //         &client_config,
        //         amount,
        //         swap_data,
        //         target_contract,
        //         asset,
        //         payload,
        //     )?;
        //     let recent_blockhash = rpc_client.get_latest_blockhash()?;
        //     let transaction = Transaction::new_signed_with_payer(
        //         &ix,
        //         Some(&payer.pubkey()),
        //         &[&payer],
        //         recent_blockhash,
        //     );
        //     let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
        //         Ok(sig) => sig,
        //         Err(err) => {
        //             println!("Error: {:?}", err);
        //             return Err(err.into());
        //         }
        //     };
        //     println!("Signature: {:?}", signature);
        // }
        CommandsName::DepositSolGateway { amount, receiver } => {
            let ix = deposit_sol_gateway_instr(&client_config, amount, receiver.0)?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::DepositSolAndCallGateway { amount, receiver } => {
            let target_contract = &client_config.gateway_transfer_native;
            let zrc20 = &client_config.sol_solana_zrc20;
            let mut payload = Vec::new();
            payload.extend_from_slice(&receiver.0);
            payload.extend_from_slice(&zrc20.0);
            let ix = deposit_sol_and_call_gateway_instr(
                &client_config,
                amount,
                target_contract.0,
                payload,
                None,
            )?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::DepositSplAndCallGateway {
            mint,
            zrc20,
            amount,
            receiver,
        } => {
            let target_contract = &client_config.gateway_transfer_native;
            let mut payload = Vec::new();
            payload.extend_from_slice(&receiver.0);
            payload.extend_from_slice(&zrc20.0);
            let ix = deposit_spl_and_call_gateway_instr(
                &client_config,
                mint,
                amount,
                target_contract.0,
                payload,
                None,
            )?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &ix,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::NewMint {
            authority,
            decimals,
            token_2022,
            enable_freeze,
            enable_close,
            enable_non_transferable,
            enable_permanent_delegate,
            rate_bps,
            default_account_state,
            transfer_fee,
            confidential_transfer_auto_approve,
        } => {
            let token_program = if token_2022 {
                spl_token_2022::id()
            } else {
                spl_token::id()
            };
            let authority = if let Some(key) = authority {
                key
            } else {
                payer.pubkey()
            };
            let freeze_authority = if enable_freeze { Some(authority) } else { None };
            let mut extensions = vec![];
            if enable_close {
                extensions.push(ExtensionInitializationParams::MintCloseAuthority {
                    close_authority: Some(authority),
                });
            }
            if enable_permanent_delegate {
                extensions.push(ExtensionInitializationParams::PermanentDelegate {
                    delegate: authority,
                });
            }
            if let Some(rate_bps) = rate_bps {
                extensions.push(ExtensionInitializationParams::InterestBearingConfig {
                    rate_authority: Some(authority),
                    rate: rate_bps,
                })
            }
            if let Some(state) = default_account_state {
                assert!(
                    enable_freeze,
                    "Token requires a freeze authority to default to frozen accounts"
                );
                let account_state = match state.as_str() {
                    "Uninitialized" => AccountState::Uninitialized,
                    "Initialized" => AccountState::Initialized,
                    "Frozen" => AccountState::Frozen,
                    _ => panic!("error default_account_state[Uninitialized, Initialized, Frozen]"),
                };
                extensions.push(ExtensionInitializationParams::DefaultAccountState {
                    state: account_state,
                })
            }
            if let Some(transfer_fee_value) = transfer_fee {
                let transfer_fee_basis_points = transfer_fee_value[0] as u16;
                let maximum_fee = transfer_fee_value[1];
                extensions.push(ExtensionInitializationParams::TransferFeeConfig {
                    transfer_fee_config_authority: Some(authority),
                    withdraw_withheld_authority: Some(authority),
                    transfer_fee_basis_points,
                    maximum_fee,
                });
            }
            if enable_non_transferable {
                extensions.push(ExtensionInitializationParams::NonTransferable);
            }
            if let Some(auto_approve) = confidential_transfer_auto_approve {
                extensions.push(ExtensionInitializationParams::ConfidentialTransferMint {
                    authority: Some(authority),
                    auto_approve_new_accounts: auto_approve,
                    auditor_elgamal_pubkey: None,
                });
            }

            let mint = Keypair::new();
            println!("Mint: {}", mint.pubkey());
            let create_and_init_instr = create_and_init_mint_instr(
                &client_config.clone(),
                token_program,
                &mint.pubkey(),
                &authority,
                freeze_authority.as_ref(),
                extensions,
                decimals,
            )?;

            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &create_and_init_instr,
                Some(&payer.pubkey()),
                &[&payer, &mint],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::NewToken {
            mint,
            authority,
            not_ata,
        } => {
            let authority = if let Some(key) = authority {
                key
            } else {
                payer.pubkey()
            };
            let mut signers = vec![&payer];
            let auxiliary_token_keypair = Keypair::new();
            let create_ata_instr = if not_ata {
                println!("{}", auxiliary_token_keypair.pubkey());
                signers.push(&auxiliary_token_keypair);
                create_and_init_auxiliary_token(
                    &client_config.clone(),
                    &auxiliary_token_keypair.pubkey(),
                    &mint,
                    &authority,
                )?
            } else {
                println!(
                    "{}",
                    spl_associated_token_account::get_associated_token_address(&authority, &mint)
                );
                let mint_account = rpc_client.get_account(&mint)?;
                create_ata_token_account_instr(
                    &client_config.clone(),
                    mint_account.owner,
                    &mint,
                    &authority,
                )?
            };

            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &create_ata_instr,
                Some(&payer.pubkey()),
                &signers,
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::MintTo {
            mint,
            to_token,
            amount,
        } => {
            let mint_account = rpc_client.get_account(&mint)?;
            let mint_to_instr = spl_token_mint_to_instr(
                &client_config.clone(),
                mint_account.owner,
                &mint,
                &to_token,
                amount,
                &payer,
            )?;
            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &mint_to_instr,
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => sig,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return Err(err.into());
                }
            };
            println!("Signature: {:?}", signature);
        }
        CommandsName::CreateLookupTable { authority } => {
            let authority = authority.unwrap_or(payer.pubkey());

            // 获取当前 slot
            let current_slot = rpc_client.get_slot()?;
            // 使用前一个 slot 作为 recent slot
            let recent_slot = current_slot - 1;
            println!("Using recent slot: {}", recent_slot);

            let (create_ix, lookup_table_address) =
                create_lookup_table_instr(authority, payer.pubkey(), recent_slot)?;

            println!("Lookup Table Address: {}", lookup_table_address);

            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[create_ix],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );

            // 先进行交易模拟
            let sim_result = rpc_client.simulate_transaction(&transaction)?;
            if let Some(err) = sim_result.value.err {
                println!("Error: {:?}", err);
                // 打印所有日志信息
                if let Some(logs) = sim_result.value.logs {
                    println!("Logs:");
                    for log in logs {
                        println!("{}", log);
                    }
                }
                return Err(err.into());
            }

            // 如果模拟成功，再执行实际交易
            let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
            println!("Signature: {}", signature);
        }
        CommandsName::ExtendLookupTable {
            lookup_table,
            authority,
            addresses,
        } => {
            let authority = authority.unwrap_or(payer.pubkey());

            let extend_ix =
                extend_lookup_table_instr(lookup_table, authority, payer.pubkey(), addresses)?;

            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[extend_ix],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );

            let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
            println!("Signature: {}", signature);
        }
        CommandsName::CheckLookupTable { lookup_table } => {
            let account = rpc_client.get_account(&lookup_table)?;
            let lookup_table_data = deserialize_lookup_table(&account.data)?;
            println!("Lookup Table Data:");
            println!("Authority: {:?}", lookup_table_data.meta.authority);
            println!(
                "Deactivation Slot: {:?}",
                lookup_table_data.meta.deactivation_slot
            );
            println!(
                "Last Extended Slot: {}",
                lookup_table_data.meta.last_extended_slot
            );
            println!("Addresses:");
            for (i, addr) in lookup_table_data.addresses.iter().enumerate() {
                println!("  {}: {}", i, addr);
            }
        }
        _ => todo!(),
    }

    Ok(())
}

fn load_cfg(client_config: &String) -> Result<ClientConfig> {
    let mut config = Ini::new();
    let _map = config.load(client_config).unwrap();
    let http_url = config.get("Global", "http_url").unwrap();
    if http_url.is_empty() {
        panic!("http_url must not be empty");
    }
    let ws_url = config.get("Global", "ws_url").unwrap();
    if ws_url.is_empty() {
        panic!("ws_url must not be empty");
    }
    let payer_path = config.get("Global", "payer_path").unwrap();
    if payer_path.is_empty() {
        panic!("payer_path must not be empty");
    }
    let admin_path = config.get("Global", "admin_path").unwrap();
    if admin_path.is_empty() {
        panic!("admin_path must not be empty");
    }
    let gateway_send_program_str = config.get("Global", "gateway_send_program").unwrap();
    if gateway_send_program_str.is_empty() {
        panic!("gateway_send_program must not be empty");
    }
    let gateway_send_program = Pubkey::from_str(&gateway_send_program_str).unwrap();
    let gateway_program_str = config.get("Global", "gateway_program").unwrap();
    if gateway_program_str.is_empty() {
        panic!("gateway_program must not be empty");
    }
    let gateway_program = Pubkey::from_str(&gateway_program_str).unwrap();
    let sol_solana_zrc20_str = config.get("Global", "sol_solana_zrc20").unwrap();
    if sol_solana_zrc20_str.is_empty() {
        panic!("sol_solana_zrc20 must not be empty");
    }
    let sol_solana_zrc20 = EvmAddress::from_str(&sol_solana_zrc20_str).unwrap();
    let gateway_transfer_native_str = config.get("Global", "gateway_transfer_native").unwrap();
    if gateway_transfer_native_str.is_empty() {
        panic!("gateway_transfer_native must not be empty");
    }
    let gateway_transfer_native = EvmAddress::from_str(&gateway_transfer_native_str).unwrap();

    Ok(ClientConfig {
        http_url,
        ws_url,
        payer_path,
        admin_path,
        gateway_send_program,
        gateway_program,
        sol_solana_zrc20,
        gateway_transfer_native,
    })
}

#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: CommandsName,
}

#[derive(Debug, Parser)]
pub enum CommandsName {
    CreateConfig {
        gateway: Pubkey,
        dodo_route_proxy: Pubkey,
    },
    CheckConfig,
    UpdateGateway {
        gateway: Pubkey,
    },
    UpdateDodoRouteProxy {
        dodo_route_proxy: Pubkey,
    },
    UpdateGasLimit {
        new_gas_limit: u64,
    },
    UpdateOwner {
        new_owner: Pubkey,
    },
    CloseConfig,
    // DepositSolAndCall {
    //     amount: u64,
    //     receiver: EvmAddress,
    // },
    // DepositAndCall {
    //     amount: u64,
    //     target_contract: Pubkey,
    //     payload: Vec<u8>,
    // },
    // DepositSwapAndCall {
    //     amount: u64,
    //     swap_data: Vec<u8>,
    //     target_contract: Pubkey,
    //     asset: Pubkey,
    //     payload: Vec<u8>,
    // },
    DepositSolGateway {
        amount: u64,
        receiver: EvmAddress,
    },
    DepositSolAndCallGateway {
        amount: u64,
        receiver: EvmAddress,
    },
    DepositSplAndCallGateway {
        mint: Pubkey,
        zrc20: EvmAddress,
        amount: u64,
        receiver: EvmAddress,
    },
    NewMint {
        #[arg(short, long, default_value = "9")]
        decimals: u8,
        authority: Option<Pubkey>,
        #[arg(short, long)]
        token_2022: bool,
        #[arg(long)]
        enable_freeze: bool,
        #[arg(long)]
        enable_close: bool,
        #[arg(long)]
        enable_non_transferable: bool,
        #[arg(long)]
        enable_permanent_delegate: bool,
        rate_bps: Option<i16>,
        default_account_state: Option<String>,
        #[arg(long)]
        transfer_fee: Option<Vec<u64>>,
        confidential_transfer_auto_approve: Option<bool>,
    },
    NewToken {
        mint: Pubkey,
        authority: Option<Pubkey>,
        #[arg(short, long)]
        not_ata: bool,
    },
    MintTo {
        mint: Pubkey,
        to_token: Pubkey,
        amount: u64,
    },
    CreateLookupTable {
        #[arg(long)]
        authority: Option<Pubkey>,
    },
    ExtendLookupTable {
        lookup_table: Pubkey,
        #[arg(long)]
        authority: Option<Pubkey>,
        addresses: Vec<Pubkey>,
    },
    CheckLookupTable {
        lookup_table: Pubkey,
    },
}

fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}

#[derive(Debug, Clone)]
pub struct EvmAddress([u8; 20]);

impl FromStr for EvmAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        if s.len() != 40 {
            return Err(anyhow::anyhow!("Invalid address length"));
        }
        let mut bytes = [0u8; 20];
        hex::decode_to_slice(s, &mut bytes)?;
        Ok(EvmAddress(bytes))
    }
}

impl From<EvmAddress> for [u8; 20] {
    fn from(addr: EvmAddress) -> [u8; 20] {
        addr.0
    }
}
