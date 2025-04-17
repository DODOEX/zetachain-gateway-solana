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
