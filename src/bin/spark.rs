use std::io::{stdin, Read};

use clap::{Arg, Command};
use itertools::Itertools;

fn main() {
    let clap_config = Command::new("Spark")
        .about("Sparklines for the terminal")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("INPUT")
                .help("The input to use, space or comma separated")
                .allow_hyphen_values(true)
                .num_args(0..)
                .use_value_delimiter(true),
        );

    let matches = clap_config.get_matches();
    if let Some(input) = matches.get_many::<String>("INPUT") {
        let numbers = input.flat_map(|s| s.parse::<f64>()).collect_vec();
        println!("{}", spark(&numbers))
    } else {
        let mut input: Vec<u8> = Vec::new();
        if stdin().read_to_end(&mut input).is_ok() {
            if let Ok(s) = String::from_utf8(input) {
                let numbers = s
                    .split_whitespace()
                    .flat_map(|s| s.split(','))
                    .flat_map(|v| v.parse::<f64>())
                    .collect::<Vec<_>>();
                println!("{}", spark(&numbers));
            }
        }
    }
}

fn spark(data: &[f64]) -> String {
    sparklines::spark(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphs_data() {
        assert_eq!(spark(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), "▁▂▃▄▅▆▇█");
    }

    #[test]
    fn equalizes_at_midtier_when_all_equal() {
        assert_eq!(spark(&[1.0, 1.0, 1.0, 1.0]), "▅▅▅▅")
    }
}
