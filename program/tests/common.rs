#[cfg(feature = "test-bpf")]
use solana_client::rpc_client::RpcClient;
use solana_program::hash::Hash;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;

#[allow(dead_code)]
pub struct Env {
    pub client: RpcClient,
    pub user: Keypair,
    pub recent_blockhash: Hash,
}

impl Env {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        let client = RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        );

        let user = Keypair::from_bytes(&[
            89, 61, 139, 211, 93, 133, 1, 223, 48, 48, 225, 41, 130, 190, 150, 113, 99, 31, 182,
            234, 148, 252, 9, 237, 231, 248, 4, 122, 35, 46, 142, 49, 194, 67, 195, 223, 140, 248,
            45, 171, 238, 145, 41, 230, 118, 18, 83, 60, 130, 228, 142, 74, 151, 98, 167, 191, 113,
            20, 185, 14, 156, 242, 207, 121,
        ])
        .unwrap();

        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");

        Self {
            client,
            user,
            recent_blockhash,
        }
    }
}
