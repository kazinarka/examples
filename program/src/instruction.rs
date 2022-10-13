use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
    GenerateVault,
    Stake {
        /// amount of tokens to stake
        #[allow(dead_code)]
        amount: u64,
    },
    Unstake,
    StakeNft,
}

impl ExampleInstruction {}
