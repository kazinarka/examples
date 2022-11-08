use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::msg;
use switchboard_v2::VrfAccountData;
use crate::error::ContractError;

const MAX_RESULT: u64 = u64::MAX;

pub fn generate_random_number_v2<'info>(accounts: &'info [AccountInfo<'info>], _program_id: &Pubkey, max_result: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    msg!("Validate init");
    if max_result > MAX_RESULT {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let vrf = VrfAccountData::new(accounts.vrf)?;
    let result_buffer = vrf.get_result()?;
    if result_buffer == [0u8; 32] {
        msg!("vrf buffer empty");
        return Ok(());
    }

    msg!("Result buffer is {:?}", result_buffer);
    let value: &[u128] = bytemuck::cast_slice(&result_buffer[..]);
    msg!("u128 buffer {:?}", value);
    let result = value[0] % max_result as u128 + 1;
    msg!("Current VRF Value [1 - {}) = {}!", max_result, result);

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'info> {
    pub vrf: &'info AccountInfo<'info>,
}

impl<'info> Accounts<'info> {
    #[allow(dead_code)]
    pub fn new(accounts: &'info [AccountInfo<'info>]) -> Result<Accounts<'info>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            vrf: next_account_info(acc_iter)?,
        })
    }
}