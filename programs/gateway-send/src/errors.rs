use anchor_lang::prelude::*;

#[error_code]
pub enum GatewayError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Route proxy call failed")]
    RouteProxyCallFailed,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
}

#[error_code]
pub enum OnRevertError {
    #[msg("Invalid data format")]
    InvalidDataFormat,
    #[msg("Revert message")]
    RevertMessage,
}

#[error_code]
pub enum OnCallError {
    #[msg("Invalid UTF-8")]
    InvalidUtf8,
    #[msg("Invalid pubkey")]
    InvalidPubkey,
    #[msg("Invalid remaining accounts")]
    InvalidRemainingAccounts,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Transfer failed")]
    TransferFailed,
    #[msg("Invalid receiver account")]
    InvalidReceiverAccount,
}
