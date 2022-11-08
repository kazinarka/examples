pub mod generate_random_number;
pub mod generate_random_number_v2;

use crate::error::ContractError;
use crate::instruction::ExampleInstruction;
use crate::processor::generate_random_number::generate_random_number;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use crate::processor::generate_random_number_v2::generate_random_number_v2;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: ExampleInstruction =
            match ExampleInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            ExampleInstruction::GenerateRandomNumber {max_result} => generate_random_number(accounts, program_id, max_result)?,
            ExampleInstruction::GenerateRandomNumberV2 {max_result} => generate_random_number_v2(accounts, program_id, max_result)?,
        };

        Ok(())
    }
}
