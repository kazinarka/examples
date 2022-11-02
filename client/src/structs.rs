use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;

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

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum TokenInstruction {
    /// generates new token mint
    GenerateToken,
    /// mint token to address
    Mint,
}
