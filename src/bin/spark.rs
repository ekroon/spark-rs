extern crate core;

use std::io::{stdin, Read};

use clap::{arg, Arg, Command};
use itertools::Itertools;

fn main() {
    let clap_config = Command::new("Spark")
        .about("Sparklines for the terminal")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!(-t --ticks <TICKS> "The characters to use for the sparkline"))
        .arg(
            Arg::new("INPUT")
                .help("The input to use, space or comma separated")
                .allow_hyphen_values(true)
                .num_args(0..)
                .use_value_delimiter(true),
        );

    let matches = clap_config.get_matches();
    let default = sparklines::TICKS.as_slice();
    let mut ticks = None;
    if let Some(input_ticks) = matches.get_one::<String>("ticks") {
        ticks = Some(input_ticks.chars().collect::<Vec<_>>());
    }
    if ticks.is_none() {
        ticks = Some(default.to_vec());
    }
    let ticks = ticks.unwrap();
    let string_spark = sparklines::StringSpark::new(ticks.as_slice());
    if let Some(input) = matches.get_many::<String>("INPUT") {
        let numbers = input.flat_map(|s| s.parse::<f64>()).collect_vec();
        println!("{}", string_spark.spark(&numbers))
    } else {
        let mut input: Vec<u8> = Vec::new();
        if stdin().read_to_end(&mut input).is_ok() {
            if let Ok(s) = String::from_utf8(input) {
                let numbers = s
                    .split_whitespace()
                    .flat_map(|s| s.split(','))
                    .flat_map(|v| v.parse::<f64>())
                    .collect::<Vec<_>>();
                println!("{}", string_spark.spark(&numbers));
            }
        }
    }
}
