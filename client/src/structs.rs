use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;

pub type Timestamp = u64;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
    GenerateVault,
    Stake {
        /// amount of tokens to stake
        #[allow(dead_code)]
        amount: u64,
    },
    Unstake {
        ///  Flag whether close the account
        #[allow(dead_code)]
        close_account: bool,
    },
    StakeNft,
    UnstakeNft,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StakeData {
    pub staker: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: Timestamp,
}
