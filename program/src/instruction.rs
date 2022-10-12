use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
    GenerateVault,
    Stake {
        /// amount of tokens to stake
        amount: u64,
    },
    Unstake,
    StakeNft,
    UnstakeNft,
}

impl ExampleInstruction {}
