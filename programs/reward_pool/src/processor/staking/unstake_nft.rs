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

pub fn unstake_nft(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    // get Clocl
    let clock = Clock::get()?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // find Stake data PDA
    let (stake_data, _) =
        Pubkey::find_program_address(&[&accounts.mint.key.to_bytes()], program_id);

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if stake_data != *accounts.stake_data_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get stake data
    let mut stake_data =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_data_info.data.borrow()) {
            data
        } else {
            return Err(ContractError::DeserializeError.into());
        };

    // get amount of tokens
    let amount = stake_data.amount;

    // find Vault PDA
    let (vault, vault_bump) = Pubkey::find_program_address(&[VAULT], program_id);

    if vault != *accounts.vault_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get associated token address for user wallet
    if &spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.mint.key,
    ) != accounts.destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // get associated token address for vault wallet
    if &spl_associated_token_account::get_associated_token_address(&vault, accounts.mint.key)
        != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // Creates associated token account for user wallet and transfer nft to it
    if accounts.destination.owner != accounts.token_program.key {
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
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

    // close associated token account at Vault PDA
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

    // if stake time is greater than 1 hour - get reward
    if ((clock.unix_timestamp as u64) - stake_data.timestamp) > REWARD_TIME {
        // create associated token account for tokens holdings
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                &accounts.payer.key,
                &accounts.payer.key,
                &accounts.internal_token.key,
            ),
            &[
                accounts.internal_token.clone(),
                accounts.assoc_internal_token.clone(),
                accounts.payer.clone(),
                accounts.token_program.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;

        // mint tokens to associated token account on user wallet
        invoke_signed(
            &spl_token::instruction::mint_to(
                &accounts.token_program.key,
                &accounts.internal_token.key,
                &accounts.assoc_internal_token.key,
                &accounts.vault_info.key,
                &[&accounts.vault_info.key],
                1,
            )?,
            &[
                accounts.internal_token.clone(),
                accounts.vault_info.clone(),
                accounts.payer.clone(),
                accounts.assoc_internal_token.clone(),
                accounts.token_program.clone(),
                accounts.rent.clone(),
            ],
            &[&[VAULT, &[vault_bump]]],
        )?;
    }

    // decrease amount of tokens staked and serialize data
    stake_data.amount = 0;
    stake_data.serialize(&mut &mut accounts.stake_data_info.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    /// Wallet
    pub payer: &'a AccountInfo<'b>,
    /// Nft mint address
    pub mint: &'a AccountInfo<'b>,
    /// Vault PDA
    pub vault_info: &'a AccountInfo<'b>,
    /// Associated token account for Vault wallet
    pub source: &'a AccountInfo<'b>,
    /// Associated token account for user wallet
    pub destination: &'a AccountInfo<'b>,
    /// Spl token program
    pub token_program: &'a AccountInfo<'b>,
    /// Solana system program
    pub sys_info: &'a AccountInfo<'b>,
    /// associated token program
    pub token_assoc: &'a AccountInfo<'b>,
    /// stake data PDA
    pub stake_data_info: &'a AccountInfo<'b>,
    /// Rent program
    pub rent: &'a AccountInfo<'b>,
    /// Internal token mint
    pub internal_token: &'a AccountInfo<'b>,
    /// Associated token account for user wallet derived to internal token
    pub assoc_internal_token: &'a AccountInfo<'b>,
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
            internal_token: next_account_info(acc_iter)?,
            assoc_internal_token: next_account_info(acc_iter)?,
        })
    }
}
