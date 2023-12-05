use clap::Parser;
use std::process;

use aoc::solve;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Which day's puzzle to solve
    #[arg(short, long)]
    day: u8,

    // Which part of the day's puzzle to solve
    #[arg(short, long, default_value = "1")]
    part: u8,

    // Path to input file
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let result = solve(args.day, args.part, args.file);
    match result {
        Ok(msg) => println!("Day {} part {} solution: {}", args.day, args.part, msg),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
