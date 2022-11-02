pub mod generate_token;

use crate::error::ContractError;
use crate::instruction::TokenInstruction;
use crate::processor::generate_token::generate_token;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: TokenInstruction = match TokenInstruction::try_from_slice(instruction_data)
        {
            Ok(insn) => insn,
            Err(err) => {
                msg!("Failed to deserialize instruction: {}", err);
                return Err(ContractError::InvalidInstructionData.into());
            }
        };

        match instruction {
            TokenInstruction::GenerateToken => generate_token(accounts)?,
        };

        Ok(())
    }
}
