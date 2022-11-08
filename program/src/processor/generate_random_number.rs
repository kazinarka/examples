use std::convert::TryInto;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::msg;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use switchboard_v2::AggregatorAccountData;

pub fn generate_random_number<'info>(accounts: &'info [AccountInfo<'info>], _program_id: &Pubkey, max_result: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let c = Clock::get().unwrap();

    let btc_val: f64 = AggregatorAccountData::new(accounts.btc_aggregator)?.get_result()?.try_into()?;
    let eth_val: f64 = AggregatorAccountData::new(accounts.eth_aggregator)?.get_result()?.try_into()?;
    let sol_val: f64 = AggregatorAccountData::new(accounts.sol_aggregator)?.get_result()?.try_into()?;

    let final_val = if c.unix_timestamp % 2 == 0 {
        (btc_val*eth_val*sol_val).round() as u128
    } else {
        (btc_val*sol_val).round() as u128
    };

    msg!("Current feed result is {}!", final_val % max_result);

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'info> {
    pub btc_aggregator: &'info AccountInfo<'info>,
    pub eth_aggregator: &'info AccountInfo<'info>,
    pub sol_aggregator: &'info AccountInfo<'info>,
}

impl<'info> Accounts<'info> {
    #[allow(dead_code)]
    pub fn new(accounts: &'info [AccountInfo<'info>]) -> Result<Accounts<'info>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            btc_aggregator: next_account_info(acc_iter)?,
            eth_aggregator: next_account_info(acc_iter)?,
            sol_aggregator: next_account_info(acc_iter)?,
        })
    }
}