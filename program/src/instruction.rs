use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
    GenerateVault,
    Stake {
        /// Amount of tokens to stake
        #[allow(dead_code)]
        amount: u64,
    },
    Unstake {
        ///  Flag whether close the account
        #[allow(dead_code)]
        close: bool,
    },
    StakeNft,
    UnstakeNft,
}

impl ExampleInstruction {}
