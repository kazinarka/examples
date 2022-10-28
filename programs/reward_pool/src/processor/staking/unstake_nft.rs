use crate::consts::{REWARD_TIME, VAULT};
use crate::error::ContractError;
use crate::state::staking::StakeData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

pub fn unstake_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    is_nft_holder: bool,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let clock = Clock::get()?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let (stake_data, _) = Pubkey::find_program_address(
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

    let mut stake_data =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_data_info.data.borrow()) {
            data
        } else {
            return Err(ContractError::DeserializeError.into());
        };

    let amount = stake_data.amount;

    let (vault, vault_bump) = Pubkey::find_program_address(&[VAULT], program_id);

    if vault != *accounts.vault_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.mint.key,
    ) != accounts.destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&vault, accounts.mint.key)
        != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.destination.owner != accounts.token_program.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                accounts.payer.key,
                accounts.payer.key,
                accounts.mint.key,
            ),
            &[
                accounts.payer.clone(),
                accounts.destination.clone(),
                accounts.payer.clone(),
                accounts.mint.clone(),
                accounts.sys_info.clone(),
                accounts.token_program.clone(),
                accounts.rent.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            accounts.token_program.key,
            accounts.source.key,
            accounts.destination.key,
            accounts.vault_info.key,
            &[],
            amount,
        )?,
        &[
            accounts.source.clone(),
            accounts.destination.clone(),
            accounts.vault_info.clone(),
            accounts.token_program.clone(),
        ],
        &[&[VAULT, &[vault_bump]]],
    )?;

    // TODO check the balance of account before closing && remove if
    if is_nft_holder {
        invoke_signed(
            &spl_token::instruction::close_account(
                accounts.token_program.key,
                accounts.source.key,
                accounts.payer.key,
                accounts.vault_info.key,
                &[],
            )?,
            &[
                accounts.source.clone(),
                accounts.payer.clone(),
                accounts.vault_info.clone(),
                accounts.token_program.clone(),
            ],
            &[&[VAULT, &[vault_bump]]],
        )?;
    }

    if ((clock.unix_timestamp as u64) - stake_data.timestamp) > REWARD_TIME {
        // TODO mint and transfer nft
    }

    stake_data.amount = 0;
    stake_data.serialize(&mut &mut accounts.stake_data_info.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub mint: &'a AccountInfo<'b>,
    pub vault_info: &'a AccountInfo<'b>,
    pub source: &'a AccountInfo<'b>,
    pub destination: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub sys_info: &'a AccountInfo<'b>,
    pub token_assoc: &'a AccountInfo<'b>,
    pub stake_data_info: &'a AccountInfo<'b>,
    pub rent: &'a AccountInfo<'b>,
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
            token_assoc: next_account_info(acc_iter)?,
            stake_data_info: next_account_info(acc_iter)?,
            rent: next_account_info(acc_iter)?,
        })
    }
}
