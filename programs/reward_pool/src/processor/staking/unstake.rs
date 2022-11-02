use crate::consts::{NFT, REQUIRED_SIZE, REWARD_TIME, VAULT};
use crate::error::ContractError;
use crate::state::staking::StakeData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn unstake(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    // get Clock
    let clock = Clock::get()?;

    // get Rent
    let rent = &Rent::from_account_info(accounts.rent)?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    // find address for Stake data PDA
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

    // get stake data
    let mut stake_data =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_data_info.data.borrow()) {
            data
        } else {
            return Err(ContractError::DeserializeError.into());
        };

    // get amount of staked tokens
    let amount = stake_data.amount;

    // find address of Vault PDA
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

    // Creates associated token account for user wallet and transfer tokens to it
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

    // if stake time is greater than 1 hour - get reward
    if ((clock.unix_timestamp as u64) - stake_data.timestamp) > REWARD_TIME {
        // create account for mint
        invoke(
            &solana_program::system_instruction::create_account(
                &accounts.payer.key,
                &accounts.nft_mint.key,
                rent.minimum_balance(REQUIRED_SIZE as usize),
                REQUIRED_SIZE,
                &accounts.token_program.key,
            ),
            &[
                accounts.nft_mint.clone(),
                accounts.payer.clone(),
                accounts.token_program.clone(),
            ],
        )?;

        // initialize mint on created account
        invoke(
            &spl_token::instruction::initialize_mint(
                &accounts.token_program.key,
                &accounts.nft_mint.key,
                &accounts.payer.key,
                Some(&accounts.payer.key),
                0,
            )?,
            &[
                accounts.nft_mint.clone(),
                accounts.payer.clone(),
                accounts.token_program.clone(),
                accounts.rent.clone(),
            ],
        )?;

        // create associated token account on user wallet for new nft
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                &accounts.payer.key,
                &accounts.payer.key,
                &accounts.nft_mint.key,
            ),
            &[
                accounts.nft_mint.clone(),
                accounts.token_account.clone(),
                accounts.payer.clone(),
                accounts.token_program.clone(),
                accounts.token_assoc.clone(),
            ],
        )?;

        // mint nft to associated token account on user wallet
        invoke(
            &spl_token::instruction::mint_to(
                &accounts.token_program.key,
                &accounts.nft_mint.key,
                &accounts.token_account.key,
                &accounts.payer.key,
                &[&accounts.payer.key],
                1,
            )?,
            &[
                accounts.nft_mint.clone(),
                accounts.payer.clone(),
                accounts.token_account.clone(),
                accounts.token_program.clone(),
                accounts.rent.clone(),
            ],
        )?;

        // find platform's nft PDA
        let (nft_pda, nft_pda_bump_seed) =
            Pubkey::find_program_address(&[NFT, &accounts.nft_mint.key.to_bytes()], program_id);

        if nft_pda != *accounts.nft_pda_info.key {
            return Err(ContractError::InvalidInstructionData.into());
        }

        // if PDA haven't been created yet - transfer required lamports and assign to program_id
        if accounts.nft_pda_info.owner != program_id {
            let required_lamports = rent
                .minimum_balance(0)
                .max(1)
                .saturating_sub(accounts.nft_pda_info.lamports());

            invoke(
                &system_instruction::transfer(accounts.payer.key, &nft_pda, required_lamports),
                &[
                    accounts.payer.clone(),
                    accounts.nft_pda_info.clone(),
                    accounts.sys_info.clone(),
                ],
            )?;

            invoke_signed(
                &system_instruction::assign(&nft_pda, program_id),
                &[accounts.nft_pda_info.clone(), accounts.sys_info.clone()],
                &[&[NFT, &accounts.nft_mint.key.to_bytes(), &[nft_pda_bump_seed]]],
            )?;
        }

        // TODO find out how to do
        // let creator = vec![
        //     mpl_token_metadata::state::Creator {
        //         address: *program_id,
        //         verified: false,
        //         share: 100,
        //     },
        //     mpl_token_metadata::state::Creator {
        //         address: *accounts.payer.key,
        //         verified: false,
        //         share: 0,
        //     },
        // ];
        //
        // invoke(
        //     &mpl_token_metadata::instruction::create_metadata_accounts_v2(
        //         *accounts.token_metadata_program.key,
        //         *accounts.metadata_account_info.key,
        //         *accounts.nft_mint.key,
        //         *accounts.payer.key,
        //         *accounts.payer.key,
        //         *accounts.payer.key,
        //         String::from("LitsLinkNFT"),
        //         String::from(SYMBOL),
        //         String::from(""),
        //         Some(creator),
        //         1,
        //         true,
        //         false,
        //         None,
        //         None,
        //     ),
        //     &[
        //         accounts.metadata_account_info.clone(),
        //         accounts.nft_mint.clone(),
        //         accounts.payer.clone(),
        //         accounts.payer.clone(),
        //         accounts.token_metadata_program.clone(),
        //         accounts.token_program.clone(),
        //         accounts.sys_info.clone(),
        //         accounts.rent.clone(),
        //     ]
        // )?;
        //
        // invoke(
        //     &mpl_token_metadata::instruction::create_master_edition_v3(
        //         *accounts.token_metadata_program.key,
        //         *accounts.master_edition.key,
        //         *accounts.nft_mint.key,
        //         *accounts.payer.key,
        //         *accounts.payer.key,
        //         *accounts.metadata_account_info.key,
        //         *accounts.payer.key,
        //         Some(0),
        //     ),
        //         &[
        //         accounts.master_edition.clone(),
        //         accounts.metadata_account_info.clone(),
        //         accounts.nft_mint.clone(),
        //         accounts.token_account.clone(),
        //         accounts.payer.clone(),
        //         accounts.rent.clone(),
        //     ]
        // )?;
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
    /// Mint of token to stake
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
    /// Associated token program
    pub token_assoc: &'a AccountInfo<'b>,
    /// Stake data PDA
    pub stake_data_info: &'a AccountInfo<'b>,
    /// Rent program
    pub rent: &'a AccountInfo<'b>,
    /// Mint nft address
    pub nft_mint: &'a AccountInfo<'b>,
    /// associated token account address for nft in user wallet
    pub token_account: &'a AccountInfo<'b>,
    /// platform's nft PDA
    pub nft_pda_info: &'a AccountInfo<'b>,
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
            nft_mint: next_account_info(acc_iter)?,
            token_account: next_account_info(acc_iter)?,
            nft_pda_info: next_account_info(acc_iter)?,
        })
    }
}
