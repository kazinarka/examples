use crate::processor::staking::stake::Accounts;
use crate::Timestamp;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StakeData {
    pub staker: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: Timestamp,
}

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
            &[&[
                &accounts.mint.key.to_bytes(),
                &accounts.payer.key.to_bytes(),
                &[stake_data_bump],
            ]],
        )?;

        invoke_signed(
            &system_instruction::assign(&stake_data, program_id),
            &[accounts.stake_data_info.clone(), accounts.sys_info.clone()],
            &[&[
                &accounts.mint.key.to_bytes(),
                &accounts.payer.key.to_bytes(),
                &[stake_data_bump],
            ]],
        )?;
    }

    Ok(())
}

/// Creates associated token account for Vault and transfer tokens to it
pub fn transfer_to_assoc(accounts: &Accounts, amount: u64) -> ProgramResult {
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
            amount,
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
