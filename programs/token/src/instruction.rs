use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum TokenInstruction {
    /// generates new token mint
    GenerateToken,
}

impl TokenInstruction {}
