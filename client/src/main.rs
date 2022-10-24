extern crate core;

mod consts;
mod structs;
mod transactions;

use crate::transactions::generate_vault::generate_vault;
use crate::transactions::say_hello::say_hello;
use crate::transactions::stake::stake;
use crate::transactions::stake_nft::stake_nft;
use crate::transactions::unstake::unstake;
use crate::transactions::unstake_nft::unstake_nft;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};

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
        .subcommand(
            SubCommand::with_name("generate_vault")
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
        .subcommand(
            SubCommand::with_name("stake")
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
                )
                .arg(
                    Arg::with_name("mint")
                        .short("m")
                        .long("mint")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("amount")
                        .short("a")
                        .long("amount")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("stake_nft")
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
                )
                .arg(
                    Arg::with_name("nft")
                        .short("n")
                        .long("nft")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("unstake")
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
                )
                .arg(
                    Arg::with_name("mint")
                        .short("m")
                        .long("mint")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("unstake_nft")
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
                )
                .arg(
                    Arg::with_name("nft")
                        .short("n")
                        .long("nft")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("say_hello") {
        say_hello(matches);
    }

    if let Some(matches) = matches.subcommand_matches("generate_vault") {
        generate_vault(matches);
    }

    if let Some(matches) = matches.subcommand_matches("stake") {
        stake(matches);
    }

    if let Some(matches) = matches.subcommand_matches("stake_nft") {
        stake_nft(matches);
    }

    if let Some(matches) = matches.subcommand_matches("unstake") {
        unstake(matches);
    }

    if let Some(matches) = matches.subcommand_matches("unstake_nft") {
        unstake_nft(matches);
    }
}
