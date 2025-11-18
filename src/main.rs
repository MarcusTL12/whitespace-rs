use std::io::{BufReader, Read};

use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source_path: String,
    stdin: FileOrStdin,
}

fn main() {
    let args = Args::parse();

    println!("{args:?}");

    let reader = BufReader::new(args.stdin.into_reader().unwrap());

    for l in reader.bytes() {
        let l = l.unwrap();

        println!("{l}");
        if l == b'q' {
            break;
        }
    }
}
