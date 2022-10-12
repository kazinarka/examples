pub mod say_hello;

use crate::error::ContractError;
use crate::instruction::ExampleInstruction;
use crate::processor::say_hello::say_hello;
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
        _accounts: &[AccountInfo],
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
            ExampleInstruction::SayHello => say_hello()?,
            _ => {}
        };

        Ok(())
    }
}
