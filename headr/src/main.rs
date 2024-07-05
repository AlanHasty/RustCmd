use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        value_name ("lines"),
        short('n'),
        long("lines"),
        default_value("10"))]
    number_lines: u64,

    /// Number bytes
    #[arg(
        short('c'),
        long("bytes"),
    )]
    number_bytes: Option<u64>,
}


fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _   => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
fn run(_args: Args) -> Result<()> {
    for filename in _args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}:{err}"),
            Ok(_) => println!("Opened {filename}"),
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}