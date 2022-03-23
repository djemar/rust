use clap::Parser;
use luhn::*;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sentence to capitalize
    #[clap(short, long)] // to use <cmd> -c,--code <CODE> syntax
    code: String,
}

fn main() {
    let args = Args::parse();

    if !Regex::new(r"((\d{4}) ){3}\d{4}").unwrap().is_match(&args.code) {
        println!("Invalid \
    format: 1234 5678 1234 5678")
    } else if is_valid(&args.code) {
        println!("Card is valid");
    } else {
        println!("Card is invalid");
    }
}