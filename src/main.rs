use caesar::caesar::brute_force;
use clap::{App, AppSettings, Arg, SubCommand};

//https://github.com/clap-rs/clap/blob/v3.0.12/examples/tutorial_builder/03_04_subcommands.rs
fn main() {
    let args = App::new("CryptoTop")
        .version("0.1")
        .about("Utility for pleasurably interacting with cryptography primitives.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("caesar")
                .about("String to solve for caesar cipher")
                .help("Solver for substitution cipher using bruteforcing")
                .arg(
                    Arg::with_name("input")
                        .help("string to bruteforce")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("base64")
                .about("Utilities for interacting with base64 encodings")
                .help("Provides utilities for base64 operations")
                .subcommand(
                    SubCommand::with_name("decode")
                        .help("Decodes entered string as base64")
                        .arg(Arg::with_name("input").help("String to decode")),
                ),
        )
        .get_matches();

    // Select Cryptotop Utility to execute based on commandline args.
    match args.subcommand() {
        ("caesar", Some(sub_matches)) => {
            println!("Args received: {:?}", sub_matches.value_of("input"));
            // TODO implement handling this result
            let _ = brute_force(sub_matches.value_of("input").unwrap());
        }
        ("base64", Some(sub_matches)) => {
            println!("Base64 Util subcommand");
            match sub_matches.subcommand() {
                ("decode", Some(bottom_matches)) => {
                    println!("Args received: {:?}", bottom_matches.value_of("input"))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
