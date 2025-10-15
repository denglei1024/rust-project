use std::{fs, path::Path};

use clap::Parser;

fn main() {
    // rwc -l input.txt
    // rwc -w input.txt
    // rwc -c input.txt
    let cli = Cli::parse();
    //verify input file exists
    if Path::new(&cli.input).exists() == false {
        eprintln!("Input file does not exist");
        std::process::exit(1);
    }
    let contents = fs::read_to_string(&cli.input)
        .expect("Something went wrong reading the file");
    // get actions
    if cli.lines {
        println!("Counting lines in {}", cli.input);
        // Count lines
        let line_count = contents.lines().count();
        println!("Line count: {}", line_count);
    }
    if cli.chars {
        println!("Counting characters in {}", cli.input);
        // Count characters
        let char_count = contents.chars().count();
        println!("Character count: {}", char_count);
    }
    if cli.bytes {
        println!("Counting bytes in {}", cli.input);
        // Count bytes
        let byte_count = contents.as_bytes().len();
        println!("Byte count: {}", byte_count);
    }
}


#[derive(Parser, Debug)]
struct Cli{
    /// Input file
    #[clap(value_parser)]
    input: String,

    /// Count lines
    #[clap(short = 'l', long = "lines", action)]
    lines: bool,

    /// Count characters
    #[clap(short = 'c', long = "chars", action)]
    chars: bool,

    /// Count bytes
    #[clap(short = 'b', long = "bytes", action)]
    bytes: bool,
}