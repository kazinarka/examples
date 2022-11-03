use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::transaction::Transaction;

pub const PROGRAM_ID: &str = "BZMNMWMcjtj3pXQihBJEjFgZpc24zo534dGuDNxGJUDd";

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    GenerateRandomNumber {
        /// max result of random number
        #[allow(dead_code)]
        max_result: u64,
    },
}

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("generate_random_number")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate_random_number") {
        let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

        let url = match matches.value_of("env") {
            Some("dev") => "https://api.devnet.solana.com",
            _ => "https://api.mainnet-beta.solana.com",
        };
        let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

        let wallet_path = matches.value_of("sign").unwrap();
        let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
        let wallet_pubkey = wallet_keypair.pubkey();

        let btc = "9AR8aftLBcspD1CKgzYBqtKQUTifYsYaA5xRSary1ntM".parse::<Pubkey>().unwrap();
        let eth = "H4ZLU7F3QMLqTkn9CM4dtTkcEtfjncvK2hacjDwmrCXv".parse::<Pubkey>().unwrap();
        let sol = "7AFybWd6zMQnkUsvpxc8CnSFUFWzVfGq9tMVxUVue1bk".parse::<Pubkey>().unwrap();

        let instructions = vec![Instruction::new_with_borsh(
            program_id,
            &ExampleInstruction::GenerateRandomNumber {max_result: 1000},
            vec![
                AccountMeta::new(btc, false),
                AccountMeta::new(eth, false),
                AccountMeta::new(sol, false),
            ],
        )];

        let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
        tx.sign(&vec![&wallet_keypair], recent_blockhash);
        let id = client.send_transaction(&tx).expect("Transaction failed.");
        println!("tx id: {:?}", id);
    }
}
