# Infrastructure

## Program
`cd program`
> program/src
- Source files for staking smart contract program

## Client
`cd client`
- Simple service to call SC instructions via command line

# Setup + Commands

## Create token

- NOTE: Only have to do this in development. In production, token should already exist and you just have to use it.

`spl-token create-token --decimals 0`

- NOTE: Any decimal spl token will work. Just using 0 for development purposes.

`spl-token create-account <mint>`

- NOTE: replace <mint> with the returned mint address from above

`spl-token mint <mint> <amount>`

## Set ADMIN const in `program/src/consts.rs` and  enter program id to declare_id macro in `program/src/lib.rs`

`make build`

- NOTE: If `cargo build-bpf` doesn't work for you, run `rm -rf ~/.cache/solana` and then re-run the build command again. This should force solana to re-download and link the bpf utilities.

## Deployment will cost 0.81975576 sol

`make deploy`

## Set PROGRAM_ID const in `client/src/consts.rs`

- NOTE: if you want to call devnet contract, just add `-e dev` to commands in Makefile

## Fill your data in commands in Makefile

## Generate vault

`make generate_vault`

## Call "SayHello" instruction

`make say_hello`

## Call "Stake" instruction

`make stake`

## Call "StakeNft" instruction

`make stake_nft`

## Call "Unstake" instruction

`make unstake`

## Call "UnstakeNft" instruction

`make unstake_nft`