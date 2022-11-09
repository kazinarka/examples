use crate::processor::staking::stake_nft::Accounts;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;

use crate::error::ContractError;

/// if PDA haven't been created yet - transfer required lamports, allocate data with size and assign to program_id
pub fn pay_rent(
    accounts: &Accounts,
    program_id: &Pubkey,
    rent: &Rent,
    stake_data: Pubkey,
    stake_data_bump: u8,
) -> ProgramResult {
    if accounts.stake_data_info.owner != program_id {
        let size: u64 = 32 + 32 + 8 + 8;

        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(accounts.stake_data_info.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &stake_data, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.stake_data_info.clone(),
                accounts.sys_info.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::allocate(&stake_data, size),
            &[accounts.stake_data_info.clone(), accounts.sys_info.clone()],
            &[&[&accounts.mint.key.to_bytes(), &[stake_data_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&stake_data, program_id),
            &[accounts.stake_data_info.clone(), accounts.sys_info.clone()],
            &[&[&accounts.mint.key.to_bytes(), &[stake_data_bump]]],
        )?;
    }

    Ok(())
}

/// Creates associated token account for Vault and transfer nft to it
pub fn transfer_nft_to_assoc(accounts: &Accounts) -> ProgramResult {
    if accounts.destination.owner != accounts.token_program.key {
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                accounts.payer.key,
                accounts.vault_info.key,
                accounts.mint.key,
            ),
            &[
                accounts.payer.clone(),
                accounts.destination.clone(),
                accounts.vault_info.clone(),
                accounts.mint.clone(),
                accounts.sys_info.clone(),
                accounts.token_program.clone(),
                accounts.rent_info.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;
    }

    invoke(
        &spl_token::instruction::transfer(
            accounts.token_program.key,
            accounts.source.key,
            accounts.destination.key,
            accounts.payer.key,
            &[],
            1,
        )?,
        &[
            accounts.source.clone(),
            accounts.destination.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
        ],
    )?;

    Ok(())
}

pub fn check_metadata_account(
    mint: &AccountInfo,
    metadata_account_info: &AccountInfo,
) -> ProgramResult {
    if &Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.to_bytes(),
            &mint.key.to_bytes(),
        ],
        &mpl_token_metadata::ID,
    )
    .0 != metadata_account_info.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    Ok(())
}