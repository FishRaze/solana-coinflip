use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameAccount {
    pub is_initialized: bool,
    pub game: Game,
}

impl Sealed for GameAccount {}

impl IsInitialized for GameAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GameAccount {
    const LEN: usize = 1 + 32 + 32 + 8 + 1 + 8 + 32 + 1;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let game_account = Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(game_account)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = self.try_to_vec().unwrap();
        dst[..data.len()].copy_from_slice(&data);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameServiceAccount {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub next_game_id: u64,
}

impl Sealed for GameServiceAccount {}

impl IsInitialized for GameServiceAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GameServiceAccount {
    const LEN: usize = 1 + 32 + 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let service_account = Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(service_account)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = self.try_to_vec().unwrap();
        dst[..data.len()].copy_from_slice(&data);
    }
} 