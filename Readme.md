# Infrastructure

## Reward pool
`cd programs/reward_pool`
> programs/reward_pool/src
- Source files for reward pool smart contract program

## Token
`cd programs/token`
> programs/token/src
- Source files for token smart contract program

## Client
`cd client`
- Simple service to call SC instructions via command line

# Setup + Commands

## Set ADMIN const in `programs/reward_pool/src/consts.rs` and `programs/token/src/consts.rs`

## Build programs
`make build`

`make token_build`

- NOTE: If `cargo build-bpf` doesn't work for you, run `rm -rf ~/.cache/solana` and then re-run the build command again. This should force solana to re-download and link the bpf utilities.

## Deploy programs

`make deploy`

## Set PROGRAM_ID const in `client/src/consts.rs`

- NOTE: if you want to call devnet contract, just add `-e dev` to commands in Makefile
- NOTE: if you want to call testnet contract, just add `-e test` to commands in Makefile

## Fill your data in commands in Makefile

## Generate vault

`make generate_vault`

## Generate token

`make generate_token`

## Set REWARD_TOKEN const in `programs/reward_pool/src/consts.rs` and `client/src/consts.rs`

## Redeploy

`make deploy`

# Staking commands

## Call "Stake" instruction

`make stake`

## Call "StakeNft" instruction

`make stake_nft`

## Call "Unstake" instruction

`make unstake`

## Call "UnstakeNft" instruction

`make unstake_nft`