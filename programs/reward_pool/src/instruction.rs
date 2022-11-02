use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    /// generates Vault PDA for token holdings
    GenerateVault,
    /// stakes tokens in Vault PDA holding
    Stake {
        /// amount of tokens to stake
        #[allow(dead_code)]
        amount: u64,
    },
    /// unstakes tokens from Vault PDA holding and mints platform's nft as the reward
    Unstake,
    /// stakes platform's nft in Vault PDA holding
    StakeNft,
    /// unstakes nft from Vault PDA holding and mints platform's tokens as the reward
    UnstakeNft,
}

impl ExampleInstruction {}
