#[cfg(feature = "test-bpf")]
mod common;

use example_program::id;
use example_program::instruction::ExampleInstruction;

use crate::common::Env;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_generate_vault() {
    let program_id = id();

    let env = Env::new().await;

    let instruction = ExampleInstruction::say_hello(program_id);

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&env.user.pubkey()));

    tx.sign(&vec![&env.user], env.recent_blockhash);

    env.client
        .send_transaction(&tx)
        .expect("Transaction failed.");
}
