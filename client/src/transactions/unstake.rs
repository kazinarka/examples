use borsh::BorshDeserialize;
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

use crate::consts::{ASSOCIATED_TOKEN, PROGRAM_ID, RENT, VAULT};
use crate::structs::{ExampleInstruction, StakeData};

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

    let close =
        {
            let token_balance = client.get_token_account_balance(&source)
                .expect("Fail to get account balance.");
            let token_balance = token_balance.amount.parse::<u64>().expect("Fail to parse balance.");

            let data = client.get_account_data(&stake_data).expect("Fail to get account data.");

            let stake_data: StakeData = match StakeData::try_from_slice(data.as_slice())
            {
                Ok(data) => data,
                Err(err) => panic!("{:?}", err),
            };
            token_balance == stake_data.amount
        };


    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &ExampleInstruction::Unstake { close_account: close },
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
        ],
    )];

    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
