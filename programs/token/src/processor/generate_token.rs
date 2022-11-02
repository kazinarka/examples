use crate::consts::{ADMIN, REQUIRED_SIZE};
use crate::error::ContractError;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn generate_token(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    // get Rent
    let rent = &Rent::from_account_info(accounts.rent_info)?;

    // admin check
    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    // create account for mint
    invoke(
        &solana_program::system_instruction::create_account(
            &accounts.payer.key,
            &accounts.token_mint.key,
            rent.minimum_balance(REQUIRED_SIZE as usize),
            REQUIRED_SIZE,
            &accounts.token_program.key,
        ),
        &[
            accounts.token_mint.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
        ],
    )?;

    // initialize mint on created account
    invoke(
        &spl_token::instruction::initialize_mint(
            &accounts.token_program.key,
            &accounts.token_mint.key,
            &accounts.mint_authority.key,
            Some(&accounts.mint_authority.key),
            0,
        )?,
        &[accounts.token_mint.clone(), accounts.rent_info.clone()],
    )?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    /// Wallet
    pub payer: &'a AccountInfo<'b>,
    /// Solana system program
    pub system_program: &'a AccountInfo<'b>,
    /// Rent program
    pub rent_info: &'a AccountInfo<'b>,
    /// token mint address
    pub token_mint: &'a AccountInfo<'b>,
    /// Spl token program
    pub token_program: &'a AccountInfo<'b>,
    /// mint authority
    pub mint_authority: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            token_mint: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            mint_authority: next_account_info(acc_iter)?,
        })
    }
}
