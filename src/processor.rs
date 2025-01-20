use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

use crate::{
    error::CoinFlipError,
    instruction::GameInstruction,
    state::{GameAccount, GameServiceAccount, GameState},
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GameInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            GameInstruction::CreateGame { bet_amount, choice } => {
                Self::process_create_game(program_id, accounts, bet_amount, choice)
            }
            GameInstruction::JoinGame { game_id } => {
                Self::process_join_game(program_id, accounts, game_id)
            }
            GameInstruction::CompleteGame { game_id, seed, result } => {
                Self::process_complete_game(program_id, accounts, game_id, seed, result)
            }
            GameInstruction::CancelGame { game_id } => {
                Self::process_cancel_game(program_id, accounts, game_id)
            }
        }
    }

    fn process_create_game(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bet_amount: u64,
        choice: bool,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let game_account = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let service_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;
        let rent = Rent::get()?;

        // Verify account ownership
        if game_account.owner != program_id {
            return Err(CoinFlipError::InvalidGameAccount.into());
        }

        // Verify service account
        if service_account.owner != program_id {
            return Err(CoinFlipError::InvalidServiceAccount.into());
        }

        // Verify bet amount
        if bet_amount == 0 {
            return Err(CoinFlipError::InvalidBetAmount.into());
        }

        // Verify player has enough funds
        if player.lamports() < bet_amount {
            return Err(CoinFlipError::InsufficientFunds.into());
        }

        // Create game account
        let space = GameAccount::LEN;
        let rent_lamports = rent.minimum_balance(space);

        // Transfer lamports for rent
        solana_program::program::invoke(
            &system_instruction::transfer(player.key, game_account.key, rent_lamports),
            &[player.clone(), game_account.clone(), system_program.clone()],
        )?;

        // Transfer bet amount
        solana_program::program::invoke(
            &system_instruction::transfer(player.key, game_account.key, bet_amount),
            &[player.clone(), game_account.clone(), system_program.clone()],
        )?;

        // Initialize game account data
        let mut game_account_data = GameAccount {
            is_initialized: true,
            game: Game {
                id: 0, // Will be set from service account
                player1: *player.key,
                player2: None,
                bet_amount,
                state: GameState::Created,
                created_at: clock.unix_timestamp,
                service_hash: [0; 32],
                player1_choice: choice,
            },
        };

        // Get next game ID from service account
        let mut service_account_data = GameServiceAccount::unpack(&service_account.data.borrow())?;
        let game_id = service_account_data.next_game_id;
        service_account_data.next_game_id += 1;
        game_account_data.game.id = game_id;

        // Save the accounts
        GameAccount::pack(game_account_data, &mut game_account.data.borrow_mut())?;
        GameServiceAccount::pack(service_account_data, &mut service_account.data.borrow_mut())?;

        msg!("Game created with ID: {}", game_id);
        Ok(())
    }

    fn process_join_game(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        game_id: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let game_account = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        // Verify account ownership
        if game_account.owner != program_id {
            return Err(CoinFlipError::InvalidGameAccount.into());
        }

        // Load game account data
        let mut game_account_data = GameAccount::unpack(&game_account.data.borrow())?;

        // Verify game state
        if game_account_data.game.state != GameState::Created {
            return Err(CoinFlipError::InvalidGameState.into());
        }

        // Verify game hasn't expired
        if clock.unix_timestamp > game_account_data.game.created_at + 3600 { // 1 hour expiry
            return Err(CoinFlipError::GameExpired.into());
        }

        // Verify player is not the creator
        if game_account_data.game.player1 == *player.key {
            return Err(CoinFlipError::NotAuthorized.into());
        }

        // Verify player has enough funds
        if player.lamports() < game_account_data.game.bet_amount {
            return Err(CoinFlipError::InsufficientFunds.into());
        }

        // Transfer bet amount
        solana_program::program::invoke(
            &system_instruction::transfer(
                player.key,
                game_account.key,
                game_account_data.game.bet_amount,
            ),
            &[player.clone(), game_account.clone(), system_program.clone()],
        )?;

        // Update game state
        game_account_data.game.player2 = Some(*player.key);
        game_account_data.game.state = GameState::Joined;

        // Save the account
        GameAccount::pack(game_account_data, &mut game_account.data.borrow_mut())?;

        msg!("Player joined game {}", game_id);
        Ok(())
    }

    fn process_complete_game(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        game_id: u64,
        seed: [u8; 32],
        result: bool,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let game_account = next_account_info(account_info_iter)?;
        let service_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Verify account ownership
        if game_account.owner != program_id {
            return Err(CoinFlipError::InvalidGameAccount.into());
        }

        // Verify service account
        if service_account.owner != program_id {
            return Err(CoinFlipError::InvalidServiceAccount.into());
        }

        // Load game account data
        let mut game_account_data = GameAccount::unpack(&game_account.data.borrow())?;

        // Verify game state
        if game_account_data.game.state != GameState::Joined {
            return Err(CoinFlipError::InvalidGameState.into());
        }

        // Verify service hash
        if game_account_data.game.service_hash != [0; 32] &&
           game_account_data.game.service_hash != seed {
            return Err(CoinFlipError::InvalidServiceHash.into());
        }

        // Calculate winner
        let winner_pubkey = if result == game_account_data.game.player1_choice {
            game_account_data.game.player1
        } else {
            game_account_data.game.player2.unwrap()
        };

        // Calculate winnings (total bet amount)
        let winnings = game_account_data.game.bet_amount * 2;

        // Transfer winnings to winner
        **game_account.try_borrow_mut_lamports()? -= winnings;
        let winner_account = next_account_info(account_info_iter)?;
        **winner_account.try_borrow_mut_lamports()? += winnings;

        // Update game state
        game_account_data.game.state = GameState::Completed;

        // Save the account
        GameAccount::pack(game_account_data, &mut game_account.data.borrow_mut())?;

        msg!("Game {} completed. Winner: {}", game_id, winner_pubkey);
        Ok(())
    }

    fn process_cancel_game(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        game_id: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let game_account = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        // Verify account ownership
        if game_account.owner != program_id {
            return Err(CoinFlipError::InvalidGameAccount.into());
        }

        // Load game account data
        let mut game_account_data = GameAccount::unpack(&game_account.data.borrow())?;

        // Verify game state
        if game_account_data.game.state != GameState::Created {
            return Err(CoinFlipError::InvalidGameState.into());
        }

        // Verify authority to cancel
        let is_creator = game_account_data.game.player1 == *player.key;
        let is_expired = clock.unix_timestamp > game_account_data.game.created_at + 3600;
        if !is_creator && !is_expired {
            return Err(CoinFlipError::NotAuthorized.into());
        }

        // Refund bet amount to creator
        let refund_amount = game_account_data.game.bet_amount;
        **game_account.try_borrow_mut_lamports()? -= refund_amount;
        **player.try_borrow_mut_lamports()? += refund_amount;

        // Update game state
        game_account_data.game.state = GameState::Cancelled;

        // Save the account
        GameAccount::pack(game_account_data, &mut game_account.data.borrow_mut())?;

        msg!("Game {} cancelled", game_id);
        Ok(())
    }
} 