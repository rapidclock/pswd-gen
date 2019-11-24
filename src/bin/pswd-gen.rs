extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};
use pswd_gen::conf;
use std::str::FromStr;

fn main() {
    let cli_matches = App::new("Random Password Generator")
        .author(crate_authors!())
        .version(crate_version!())
        .version_short("v")
        .bin_name("pswd-gen")
        .long_about(
            "This is a Configurable Random Password Generator, which generates a random \
             password of a specified size. The composition of the randomly generated string \
             can be configured with the options of the CLI Tool. Check the help menu for available \
             options.",
        )
        .about("A Configurable Random Password Generator")
        .after_help("Provided as-is to use according to the MIT Licensing terms\n")
        .arg(
            Arg::with_name("lower")
                .help("Allows the presence of lowercase alphabets in the generated string")
                .short("l")
                .long("lower-case"),
        )
        .arg(
            Arg::with_name("upper")
                .help("Allows the presence of uppercase alphabets in the generated string")
                .short("u")
                .long("upper-case"),
        )
        .arg(
            Arg::with_name("digits")
                .help("Allows the presence of digits in the generated string")
                .short("d")
                .long("digits"),
        )
        .arg(
            Arg::with_name("symbols")
                .help("Allows the presence of ascii symbols in the generated string")
                .short("y")
                .long("symbols")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .help("Used to specify the desired size of the generated password")
                .short("s")
                .long("req-size")
                .default_value("12")
                .takes_value(true),
        )
        .get_matches();

    let mut req_len: usize = 0;

    if let Some(req_len_as_str) = cli_matches.value_of("size") {
        req_len = usize::from_str(req_len_as_str).unwrap();
    }

    let mut values: Option<Vec<char>> = None;
    if let Some(symbols) = cli_matches.value_of("symbols") {
        if !symbols.is_empty() {
            values = Some(symbols.chars().map(|c| {
                if c.is_ascii() {
                    c
                } else {
                    eprintln!("Non-ascii value passed as a symbol. The option only supports non-ascii symbols");
                    std::process::exit(1);
                }
            }).collect());
        } else {
            eprintln!(
                "Empty string given as option for symbols. Please specify atleast one symbol."
            );
            std::process::exit(1);
        }
    }

    let config = conf::PasswordConfig::with_params(
        req_len,
        cli_matches.is_present("lower"),
        cli_matches.is_present("upper"),
        cli_matches.is_present("digits"),
        values,
    );
    println!("{}", pswd_gen::make_new_password(config));
}
