use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    GenerateVault,
    Stake {
        /// amount of tokens to stake
        #[allow(dead_code)]
        amount: u64,
    },
    Unstake,
    StakeNft,
    UnstakeNft,
}

impl ExampleInstruction {}
