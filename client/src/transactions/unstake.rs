use crate::consts::{ASSOCIATED_TOKEN, PROGRAM_ID, RENT, VAULT};
use crate::structs::ExampleInstruction;
use clap::ArgMatches;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use spl_token::state::{Account, Mint};
use spl_token_metadata::state::{PREFIX, EDITION};

pub fn unstake(matches: &ArgMatches) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    let url = match matches.value_of("env") {
        Some("dev") => "https://api.devnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let mint = matches.value_of("mint").unwrap().parse::<Pubkey>().unwrap();

    let (vault, _p) = Pubkey::find_program_address(&[VAULT], &program_id);

    let destination =
        spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &mint);

    let source = spl_associated_token_account::get_associated_token_address(&vault, &mint);

    let (stake_data, _) =
        Pubkey::find_program_address(&[&mint.to_bytes(), &wallet_pubkey.to_bytes()], &program_id);
    println!("{:?}", wallet_pubkey);

    let nft_mint = create_mint_account(&wallet_keypair, &client);
    let token_account = create_token_account(&wallet_keypair, &client);

    let (metadata, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &nft_mint.to_bytes(),
        ],
        &spl_token_metadata::ID,
    );

    let (master_edition, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &nft_mint.to_bytes(),
            EDITION.as_bytes(),
        ],
        &spl_token_metadata::ID,
    );

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &ExampleInstruction::Unstake,
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new(source, false),
            AccountMeta::new(destination, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN.parse::<Pubkey>().unwrap(), false),
            AccountMeta::new(stake_data, false),
            AccountMeta::new_readonly(RENT.parse::<Pubkey>().unwrap(), false),
            AccountMeta::new_readonly(spl_token_metadata::ID, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new(nft_mint, false),
            AccountMeta::new(master_edition, false),
            AccountMeta::new(token_account, false),
        ],
    )];

    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}

fn create_mint_account(wallet_keypair: &Keypair, client: &RpcClient) -> Pubkey {
    let mint_account: Keypair = Keypair::new();
    let mint_account_pubkey = mint_account.pubkey();
    let wallet_pubkey = wallet_keypair.pubkey();

    let minimum_balance_for_rent_exemption = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .unwrap();

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &wallet_pubkey,
        &mint_account_pubkey,
        minimum_balance_for_rent_exemption,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let latest_blockhash = client.get_latest_blockhash().unwrap();

    let transaction: Transaction = Transaction::new_signed_with_payer(
        &vec![create_account_instruction],
        Some(&wallet_pubkey),
        &[&mint_account, &wallet_keypair],
        latest_blockhash,
    );

    let result = client.send_and_confirm_transaction_with_spinner(&transaction);

    if result.is_ok() {
        println!(
            "Successfully created a Mint Account with Pubkey: {:?}",
            mint_account_pubkey
        )
    };

    return mint_account_pubkey;
}

fn create_token_account(
    wallet_keypair: &Keypair,
    client: &RpcClient,
) -> Pubkey {
    let wallet_pubkey = wallet_keypair.pubkey();
    let account_mint_to: Keypair = Keypair::new();
    let account_mint_to_pubkey: Pubkey = account_mint_to.pubkey();

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &wallet_pubkey,
        &account_mint_to_pubkey,
        client
            .get_minimum_balance_for_rent_exemption(Account::LEN)
            .unwrap(),
        Account::LEN as u64,
        &spl_token::id(),
    );

    let latest_blockhash = client.get_latest_blockhash().unwrap();

    let transaction: Transaction = Transaction::new_signed_with_payer(
        &vec![create_account_instruction],
        Some(&wallet_pubkey),
        &[&wallet_keypair, &account_mint_to],
        latest_blockhash,
    );

    let result = client.send_and_confirm_transaction_with_spinner(&transaction);
    if result.is_ok() {
        println!(
            "Successfully created a Token Account with Pubkey: {:?}",
            account_mint_to_pubkey
        )
    };

    return account_mint_to_pubkey;
}