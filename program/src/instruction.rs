use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    GenerateRandomNumber {
        /// max result of random number
        #[allow(dead_code)]
        max_result: u64,
    },
    GenerateRandomNumberV2 {
        /// max result of random number
        #[allow(dead_code)]
        max_result: u64,
    },
}

impl ExampleInstruction {}
