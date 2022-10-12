use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::transaction::Transaction;

pub const PROGRAM_ID: &str = "4T7xMGLUSPhpnAou6TiZDB9XCDHgGx1u7hDbGWdXnfmG";

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    SayHello,
}

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("say_hello")
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

    if let Some(matches) = matches.subcommand_matches("say_hello") {
        let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

        let url = match matches.value_of("env") {
            Some("dev") => "https://api.devnet.solana.com",
            _ => "https://api.mainnet-beta.solana.com",
        };
        let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

        let wallet_path = matches.value_of("sign").unwrap();
        let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
        let wallet_pubkey = wallet_keypair.pubkey();

        let instructions = vec![Instruction::new_with_borsh(
            program_id,
            &ExampleInstruction::SayHello,
            vec![],
        )];

        let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
        tx.sign(&vec![&wallet_keypair], recent_blockhash);
        let id = client.send_transaction(&tx).expect("Transaction failed.");
        println!("tx id: {:?}", id);
    }
}
