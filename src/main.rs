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

    let tmp = args.stdin.contents().unwrap();

    println!("{tmp}");
}
