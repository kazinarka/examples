use crate::consts::{ASSOCIATED_TOKEN, PROGRAM_ID, RENT, REWARD_TOKEN, VAULT};
use crate::structs::ExampleInstruction;
use clap::ArgMatches;
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

pub fn unstake_nft(matches: &ArgMatches) {
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

    // get mint
    let mint = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();

    // find Vault PDA
    let (vault, _) = Pubkey::find_program_address(&[VAULT], &program_id);

    // get associated token address of user wallet
    let destination =
        spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &mint);

    // get associated token address of vault wallet
    let source = spl_associated_token_account::get_associated_token_address(&vault, &mint);

    // find stake data PDA
    let (stake_data, _) = Pubkey::find_program_address(&[&mint.to_bytes()], &program_id);

    // get reward token mint
    let internal_token = REWARD_TOKEN.parse::<Pubkey>().unwrap();

    // get associated token address of user wallet derived to reward token
    let assoc_internal_token =
        spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &internal_token);

    // construct instruction
    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &ExampleInstruction::UnstakeNft,
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
            AccountMeta::new(internal_token, false),
            AccountMeta::new(assoc_internal_token, false),
        ],
    )];
    // generate transaction
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    // get recent blockhash
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    // sign transaction
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    // send transaction
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
