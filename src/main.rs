use base64::base64;
use caesar::caesar::brute_force;
use clap::{App, AppSettings, Arg, SubCommand};
use english_recognition::frequency_analysis::score_strings;
use hex::hex;

//https://github.com/clap-rs/clap/blob/v3.0.12/examples/tutorial_builder/03_04_subcommands.rs
fn main() {
    let args = App::new("CryptoTop")
        .version("0.1")
        .about("A service top for a bratty crypto problem near you.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("caesar")
                .about("String to solve for caesar cipher")
                .help("Solver for substitution cipher using bruteforcing")
                .arg(
                    Arg::with_name("input")
                        .help("string to bruteforce")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("base64")
                .about("Utilities for interacting with base64 encodings")
                .help("Provides utilities for base64 operations")
                .subcommand(
                    SubCommand::with_name("decode")
                        .help("Decodes entered string as base64")
                        .arg(Arg::with_name("input").help("Data to decode")),
                )
                .subcommand(
                    SubCommand::with_name("encode")
                        .help("Encodes entered bytes as base64")
                        .arg(Arg::with_name("input").help("Data to encode")),
                ),
        )
        .subcommand(
            SubCommand::with_name("hex")
                .about("Decode and Encode hex strings")
                .subcommand(
                    SubCommand::with_name("decode")
                        .help("Enter hex string to decode")
                        .arg(Arg::with_name("input").help("Hex data to decode")),
                )
                .subcommand(
                    SubCommand::with_name("encode")
                        .help("Enter string or data to encode")
                        .arg(Arg::with_name("input").help("Data to encode")),
                ),
        )
        .get_matches();

    // Select Cryptotop Utility to execute based on commandline args.
    match args.subcommand() {
        // HANDLE CAESAR SUB COMMANDS
        ("caesar", Some(sub_matches)) => {
            // bruteforce string
            let res = brute_force(sub_matches.value_of("input").unwrap());

            // order string by most likely english lang string
            let res = score_strings(res);

            println!(
                "Brute force results for: {}\n",
                sub_matches.value_of("input").unwrap()
            );

            println!("Ordered by english language frequency score:");
            let mut first = true;
            for (can, score) in res {
                if first {
                    println!("Highest score of {score} for:\n\t{can}\n");
                    first = false;
                } else {
                    println!("{can}");
                }
            }
        }

        // HANDLE BASE64 SUB COMMANDS
        ("base64", Some(sub_matches)) => match sub_matches.subcommand() {
            ("decode", Some(bottom_matches)) => {
                let input = bottom_matches.value_of("input").unwrap();
                let decoded = base64::decode(input.trim());

                let attempt = String::from_utf8(decoded);

                match attempt {
                    Ok(good) => println!("{good}"),
                    Err(_) => println!("Error displaying decoded"),
                };
            }
            ("encode", Some(bottom_matches)) => {
                let input = bottom_matches.value_of("input").unwrap();
                println!("{}", base64::encode(Vec::from(input)))
            }
            _ => unreachable!(),
        },

        // handle hex subcommand
        ("hex", Some(sub_matches)) => match sub_matches.subcommand() {
            ("decode", Some(bottom_matches)) => {
                let input = bottom_matches.value_of("input").unwrap();
                let decoded = hex::decode(input);

                let attempt = String::from_utf8(decoded.clone());
                match attempt {
                    Ok(x) => println!("{}", x),
                    Err(_) => println!("{:?}", decoded),
                }
            }
            ("encode", Some(bottom_matches)) => {
                let input = bottom_matches.value_of("input").unwrap();
                println!("{}", hex::encode(Vec::<u8>::from(input)));
            }
            _ => unreachable!(),
        },

        // should never be reached due to use of CLAP.
        _ => unreachable!(),
    }
}
