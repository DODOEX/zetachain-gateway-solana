use anchor_lang::prelude::*;

#[error_code]
pub enum GatewayError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Route proxy call failed")]
    RouteProxyCallFailed,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid data format")]
    InvalidDataFormat,
    #[msg("Revert message")]
    RevertMessage,
    #[msg("Invalid remaining accounts")]
    InvalidRemainingAccounts,
    #[msg("Invalid UTF-8")]
    InvalidUtf8,
    #[msg("Invalid pubkey")]
    InvalidPubkey,
    #[msg("Transfer failed")]
    TransferFailed,
    #[msg("Invalid receiver account")]
    InvalidReceiverAccount,
}
