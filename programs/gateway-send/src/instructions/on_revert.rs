use {
    crate::{errors::OnRevertError, states::config::ConnectedPda},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct OnRevert<'info> {
    #[account(mut, seeds = [b"connected"], bump)]
    pub pda: Account<'info, ConnectedPda>,

    /// CHECK: This is test program.
    pub gateway_pda: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn on_revert(ctx: Context<OnRevert>, amount: u64, sender: Pubkey, data: Vec<u8>) -> Result<()> {
    let pda = &mut ctx.accounts.pda;

    // Store the sender's public key
    pda.last_revert_sender = sender;

    // Convert data to a string and store it
    let message = String::from_utf8(data).map_err(|_| OnRevertError::InvalidDataFormat)?;
    pda.last_revert_message = message;

    // Transfer some portion of lamports transferred from gateway to another account
    // Check if the message contains "revert" and return an error if so
    if pda.last_revert_message.contains("revert") {
        msg!(
            "Reverting transaction due to message: '{}'",
            pda.last_revert_message
        );
        return Err(OnRevertError::RevertMessage.into());
    }

    msg!(
        "On revert executed with amount {}, sender {:?} and message {}",
        amount,
        pda.last_revert_sender,
        pda.last_revert_message
    );

    Ok(())
}
