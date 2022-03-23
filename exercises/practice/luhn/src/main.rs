use clap::Parser;
use luhn::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sentence to capitalize
    #[clap(short, long)] // to use <cmd> -c,--code <CODE> syntax
    code: String,
}

fn main() {
    let args = Args::parse();
    println!("Card: {0} is {1}", args.code, is_valid(&args.code));
}