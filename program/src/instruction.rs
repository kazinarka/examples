use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
}

impl ExampleInstruction {
    pub fn say_hello(program_id: Pubkey) -> Instruction {
        Instruction::new_with_borsh(program_id, &ExampleInstruction::SayHello, vec![])
    }
}
