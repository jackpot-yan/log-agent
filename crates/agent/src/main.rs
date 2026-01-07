use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    input_file: PathBuf,

    #[arg(short, long)]
    output_tcp: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.input_file);
    println!("{:?}", cli.output_tcp);
}
