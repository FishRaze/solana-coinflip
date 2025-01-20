#![cfg_attr(not(feature = "no-entrypoint"), feature(custom_test_frameworks))]

use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "CoinFlip",
    project_url: "https://github.com/FishRaze/solana-coinflip",
    contacts: "https://github.com/FishRaze",
    policy: "https://github.com/FishRaze/solana-coinflip/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/FishRaze/solana-coinflip",
    source_release: "v1.0.0",
    encryption: "Not yet available",
    auditors: "None",
    acknowledgements: "None",
    expiry: "2025-01-20"
}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Program entrypoint
entrypoint!(process_instruction);

// Game state enum
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum GameState {
    Created,
    Joined,
    Completed,
    Cancelled,
}

// Game structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    pub id: u64,
    pub player1: Pubkey,
    pub player2: Option<Pubkey>,
    pub bet_amount: u64,
    pub state: GameState,
    pub created_at: i64,
    pub service_hash: [u8; 32],
    pub player1_choice: bool,
}

// Program instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GameInstruction {
    CreateGame {
        bet_amount: u64,
        choice: bool,
    },
    JoinGame {
        game_id: u64,
    },
    CompleteGame {
        game_id: u64,
        seed: [u8; 32],
        result: bool,
    },
    CancelGame {
        game_id: u64,
    },
}

// Program entry point
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("CoinFlip program entry point");

    let instruction = GameInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        GameInstruction::CreateGame { bet_amount, choice } => {
            msg!("Creating new game");
            // TODO: Implement game creation logic
            Ok(())
        }
        GameInstruction::JoinGame { game_id } => {
            msg!("Joining game {}", game_id);
            // TODO: Implement game joining logic
            Ok(())
        }
        GameInstruction::CompleteGame { game_id, seed, result } => {
            msg!("Completing game {}", game_id);
            // TODO: Implement game completion logic
            Ok(())
        }
        GameInstruction::CancelGame { game_id } => {
            msg!("Cancelling game {}", game_id);
            // TODO: Implement game cancellation logic
            Ok(())
        }
    }
}

// Error type for the CoinFlip program
#[derive(Debug, thiserror::Error)]
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
}

impl From<CoinFlipError> for ProgramError {
    fn from(e: CoinFlipError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

