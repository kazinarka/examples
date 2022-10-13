pub mod generate_vault;
pub mod say_hello;
pub mod staking;

use crate::error::ContractError;
use crate::instruction::ExampleInstruction;
use crate::processor::generate_vault::generate_vault;
use crate::processor::say_hello::say_hello;
use crate::processor::staking::stake::stake;
use crate::processor::staking::unstake::unstake;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
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
            ExampleInstruction::GenerateVault => generate_vault(accounts, program_id)?,
            ExampleInstruction::Stake { amount } => stake(accounts, program_id, amount)?,
            ExampleInstruction::StakeNft => stake(accounts, program_id, 1)?,
            ExampleInstruction::Unstake => unstake(accounts, program_id, false)?,
            ExampleInstruction::UnstakeNft => unstake(accounts, program_id, true)?,
        };

        Ok(())
    }
}
