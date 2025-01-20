use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

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

impl GameInstruction {
    pub fn create_game(
        program_id: &Pubkey,
        game_account: &Pubkey,
        player: &Pubkey,
        service_account: &Pubkey,
        bet_amount: u64,
        choice: bool,
    ) -> Instruction {
        let data = GameInstruction::CreateGame {
            bet_amount,
            choice,
        }
        .try_to_vec()
        .unwrap();

        Instruction {
            program_id: *program_id,
            accounts: vec![
                AccountMeta::new(*game_account, false),
                AccountMeta::new(*player, true),
                AccountMeta::new_readonly(*service_account, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data,
        }
    }

    pub fn join_game(
        program_id: &Pubkey,
        game_account: &Pubkey,
        player: &Pubkey,
        game_id: u64,
    ) -> Instruction {
        let data = GameInstruction::JoinGame { game_id }
            .try_to_vec()
            .unwrap();

        Instruction {
            program_id: *program_id,
            accounts: vec![
                AccountMeta::new(*game_account, false),
                AccountMeta::new(*player, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data,
        }
    }

    pub fn complete_game(
        program_id: &Pubkey,
        game_account: &Pubkey,
        service_account: &Pubkey,
        game_id: u64,
        seed: [u8; 32],
        result: bool,
    ) -> Instruction {
        let data = GameInstruction::CompleteGame {
            game_id,
            seed,
            result,
        }
        .try_to_vec()
        .unwrap();

        Instruction {
            program_id: *program_id,
            accounts: vec![
                AccountMeta::new(*game_account, false),
                AccountMeta::new(*service_account, true),
            ],
            data,
        }
    }

    pub fn cancel_game(
        program_id: &Pubkey,
        game_account: &Pubkey,
        player: &Pubkey,
        game_id: u64,
    ) -> Instruction {
        let data = GameInstruction::CancelGame { game_id }
            .try_to_vec()
            .unwrap();

        Instruction {
            program_id: *program_id,
            accounts: vec![
                AccountMeta::new(*game_account, false),
                AccountMeta::new(*player, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data,
        }
    }
} 