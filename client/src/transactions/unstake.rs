use crate::consts::{ASSOCIATED_TOKEN, PROGRAM_ID, RENT, VAULT};
use crate::structs::ExampleInstruction;
use clap::ArgMatches;
use mpl_token_metadata::state::{EDITION, PREFIX};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;

pub fn unstake(matches: &ArgMatches) {
    // get program id of smart contract
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    // choose url of solana cluster
    let url = match matches.value_of("env") {
        Some("dev") => "https://api.devnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    // get client
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    // get wallet keypair
    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    // get token mint address
    let mint = matches.value_of("mint").unwrap().parse::<Pubkey>().unwrap();

    // find vault PDA
    let (vault, _p) = Pubkey::find_program_address(&[VAULT], &program_id);

    // get associated token address of user wallet
    let destination =
        spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &mint);

    // get associated token address of vault wallet
    let source = spl_associated_token_account::get_associated_token_address(&vault, &mint);

    // get stake data PDA
    let (stake_data, _) =
        Pubkey::find_program_address(&[&mint.to_bytes(), &wallet_pubkey.to_bytes()], &program_id);

    // generate mint keypair
    let nft_mint = Keypair::new();

    // get associated token address of user wallet, associated with newly generated mint
    let token_account = spl_associated_token_account::get_associated_token_address(
        &wallet_pubkey,
        &nft_mint.pubkey(),
    );

    // find metadata account
    let (metadata, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            &mpl_token_metadata::ID.to_bytes(),
            &nft_mint.pubkey().to_bytes(),
        ],
        &mpl_token_metadata::ID,
    );

    // find master edition account
    let (master_edition, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            &mpl_token_metadata::ID.to_bytes(),
            &nft_mint.pubkey().to_bytes(),
            EDITION.as_bytes(),
        ],
        &mpl_token_metadata::ID,
    );

    // construct instruction
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
            AccountMeta::new(nft_mint.pubkey(), true),
            AccountMeta::new(token_account, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(mpl_token_metadata::id(), false),
            AccountMeta::new(master_edition, false),
        ],
    )];
    // generate transaction
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    // get recent blockhash
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    // sign transaction
    tx.sign(&vec![&wallet_keypair, &nft_mint], recent_blockhash);
    // send transaction
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
