use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CoinFlipError {
    #[error("Invalid game state")]
    InvalidGameState,
    #[error("Invalid bet amount")]
    InvalidBetAmount,
    #[error("Game not found")]
    GameNotFound,
    #[error("Not authorized")]
    NotAuthorized,
    #[error("Game expired")]
    GameExpired,
    #[error("Invalid service hash")]
    InvalidServiceHash,
    #[error("Invalid game account")]
    InvalidGameAccount,
    #[error("Invalid service account")]
    InvalidServiceAccount,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account already initialized")]
    AlreadyInitialized,
}

impl From<CoinFlipError> for ProgramError {
    fn from(e: CoinFlipError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 