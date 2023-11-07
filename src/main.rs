use clap::Parser;
use std::io::Read;
use std::process::exit;

mod cli;
use cha_rs::extract;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle
        .read_to_end(&mut input)
        .expect("unable to read input");
    let s = String::from_utf8_lossy(&input);
    let s = s.trim();

    match extract(s, &cli.characters) {
        Ok(res) => {
            for (idx, c) in cli.characters.iter().zip(res) {
                println!("{} => {}", idx, c);
            }
        }
        Err(e) => {
            eprint!("error: {:?}", e);
            exit(1);
        }
    }
}
