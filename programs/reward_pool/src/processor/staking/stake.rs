use crate::consts::VAULT;
use crate::error::ContractError;
use crate::state::staking::{pay_rent, transfer_to_assoc, StakeData};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn stake(accounts: &[AccountInfo], program_id: &Pubkey, amount: u64) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    // get Clock
    let clock = Clock::get()?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get Rent
    let rent = &Rent::from_account_info(accounts.rent_info)?;

    // find address for Stake data PDA
    let (stake_data, stake_data_bump) = Pubkey::find_program_address(
        &[
            &accounts.mint.key.to_bytes(),
            &accounts.payer.key.to_bytes(),
        ],
        program_id,
    );

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if stake_data != *accounts.stake_data_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // pay rent for PDA
    pay_rent(&accounts, program_id, rent, stake_data, stake_data_bump)?;

    // check for current amount of tokens
    let current_amount =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_data_info.data.borrow()) {
            data.amount
        } else {
            0
        };

    // generate new data and serialize it to PDA
    let stake_struct = StakeData {
        staker: *accounts.payer.key,
        mint: *accounts.mint.key,
        amount: current_amount + amount,
        timestamp: clock.unix_timestamp as u64,
    };
    stake_struct.serialize(&mut &mut accounts.stake_data_info.data.borrow_mut()[..])?;

    // find Vault address
    let (vault, _vault_bump) = Pubkey::find_program_address(&[VAULT], program_id);

    if vault != *accounts.vault_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get associated token address for payer wallet
    if &spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.mint.key,
    ) != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get associated token address for vault wallet
    if &spl_associated_token_account::get_associated_token_address(&vault, accounts.mint.key)
        != accounts.destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // transfer token from payer to vault
    transfer_to_assoc(&accounts, amount)?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    /// Wallet
    pub payer: &'a AccountInfo<'b>,
    /// Mint of token to stake
    pub mint: &'a AccountInfo<'b>,
    /// Vault PDA
    pub vault_info: &'a AccountInfo<'b>,
    /// Associated token account for User wallet
    pub source: &'a AccountInfo<'b>,
    /// Associated token account for Vault wallet
    pub destination: &'a AccountInfo<'b>,
    /// Spl token program
    pub token_program: &'a AccountInfo<'b>,
    /// Solana system program
    pub sys_info: &'a AccountInfo<'b>,
    /// Rent program
    pub rent_info: &'a AccountInfo<'b>,
    /// Associated token program
    pub token_assoc: &'a AccountInfo<'b>,
    /// Stake data PDA
    pub stake_data_info: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            mint: next_account_info(acc_iter)?,
            vault_info: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            destination: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            sys_info: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
            stake_data_info: next_account_info(acc_iter)?,
        })
    }
}
