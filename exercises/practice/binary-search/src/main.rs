use std::io::{stdin, stdout, Write};
use clap::Parser;
use binary_search::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    /// Sentence to capitalize
    //#[clap(short, long)] // to use <cmd> -c,--code <CODE> syntax
    key: i32,
}

fn main() {
    let args = Args::parse();
    let mut s: String = String::new();
    let mut array = Vec::<i32>::new();

    while stdin().read_line(&mut s).unwrap() != 0 {
        // println!(">>> {}", s.trim());
        array.push(s.trim().parse::<i32>().unwrap());
        s.clear(); // svuota il buffer di stdin prima di leggere una nuova stringa
        stdout().flush().unwrap();
    };

    let index = find(&array, args.key);
    if index.is_some() {
        println!("Value {0} found at {1}.", args.key, index.unwrap());
    } else { println!("Value {} not found.", args.key)}
}