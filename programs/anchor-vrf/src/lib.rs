pub mod actions;
pub use actions::*;

pub use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::clock;
pub use anchor_spl::token::{Token, TokenAccount};
pub use switchboard_v2::{
    OracleQueueAccountData, PermissionAccountData, SbState, VrfAccountData, VrfRequestRandomness,
};

declare_id!("9rQm8b1ynr4kww3mA8AbhESnvQXgoPAQMgTih8t4pkoD");

const STATE_SEED: &[u8] = b"STATE";

#[program]
pub mod anchor_vrf {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn request_randomness(ctx: Context<RequestRandomness>, params: RequestRandomnessParams) -> Result<()> {
        RequestRandomness::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_client(ctx: Context<InitClient>, params: InitClientParams) -> Result<()> {
        InitClient::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn consume_randomness(ctx: Context<ConsumeRandomness>, params: ConsumeRandomnessParams) -> Result<()> {
        ConsumeRandomness::actuate(&ctx, &params)
    }
}

#[repr(packed)]
#[account(zero_copy)]
#[derive(Default)]
pub struct VrfClientState {
    pub bump: u8,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
    pub vrf: Pubkey,
}

#[event]
pub struct VrfClientUpdated {
    pub vrf_client: Pubkey,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
}

#[event]
pub struct RandomnessRequested {
    pub vrf_client: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct VrfClientCreated {
    pub vrf_client: Pubkey,
    pub timestamp: i64,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum AnchorVrfErrorCode {
    #[msg("Switchboard VRF Account's authority should be set to the client's state pubkey")]
    InvalidVrfAuthorityError,
    #[msg("Invalid VRF account provided.")]
    InvalidVrfAccount,
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
}