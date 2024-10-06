use std::{collections::HashMap, process::exit};

use clap::Parser;
use regex::{self, Regex};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List of roman numbers to convert into decimal
    #[arg(required = true)]
    numbers: Vec<String>,
}

fn main() {
    let roman_number: HashMap<&str, i16> = vec![
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ]
    .into_iter()
    .collect();
    let roman_re = Regex::new(r"(?m)^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$")
        .expect("invalid regex");

    let cli = Args::parse();

    for seq in cli.numbers {
        if !roman_re.is_match(&seq) {
            println!("{seq} => invalid roman sequence");
            exit(1)
        }

        match get_total(&seq, &roman_number) {
            Ok(v) => println!("{seq} => {v}"),
            Err(e) => println!("{e}"),
        }
    }
}

fn get_total(roman: &str, valid_numbers: &HashMap<&str, i16>) -> Result<i16, &'static str> {
    let mut total = 0i16;
    let mut rom_chars = roman.chars();
    while let Some(c) = rom_chars.next() {
        let current_rom_num = valid_numbers.get(c.to_string().as_str()).unwrap();
        let n = rom_chars.next();
        if n.is_none() {
            return Ok(total + current_rom_num);
        }

        let next_rom_num = valid_numbers.get(n.unwrap().to_string().as_str()).unwrap();

        total += if current_rom_num < next_rom_num {
            next_rom_num - current_rom_num
        } else {
            current_rom_num + next_rom_num
        };
    }

    Ok(total)
}
