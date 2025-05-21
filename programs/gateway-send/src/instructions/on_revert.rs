use {
    crate::{
        errors::OnRevertError,
        instructions::{decode_bytes32, SOL},
        states::{
            config::{Config, ConnectedPda},
            events::EddyCrossChainRevert,
        },
        CONFIG_SEED,
    },
    anchor_lang::prelude::*,
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

    pub system_program: Program<'info, System>,
}

pub fn on_revert(
    ctx: Context<OnRevert>,
    amount: u64,
    _sender: [u8; 20],
    data: Vec<u8>,
) -> Result<()> {
    // msg!(
    //     "on_revert {} {} {}",
    //     amount,
    //     hex::encode(_sender),
    //     data.len()
    // );
    // msg!(
    //     "remaining accounts length: {}",
    //     ctx.remaining_accounts.len()
    // );
    // let mut offset = 0;
    // let external_id = decode_bytes32(&data, &mut offset);
    // let data_sender = Pubkey::new_from_array(decode_bytes32(&data, &mut offset));

    // // check balance
    // if ctx.accounts.config.to_account_info().lamports() < amount {
    //     return Err(OnRevertError::InsufficientBalance.into());
    // }
    // // transfer sol
    // ctx.accounts.config.sub_lamports(amount).unwrap();
    // ctx.remaining_accounts[0].add_lamports(amount).unwrap();

    // msg!(
    //     "EddyCrossChainRevert {} {} {} {} {}",
    //     hex::encode(external_id),
    //     SOL,
    //     amount,
    //     data_sender,
    //     sender
    // );

    // emit!(EddyCrossChainRevert {
    //     external_id,
    //     token: SOL,
    //     amount,
    //     wallet_address: data_sender,
    // });

    Ok(())
}
