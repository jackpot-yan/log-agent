use clap::Parser;
use common::error::{Error, Result};
use input::files::FileSource;
use output::console::{Console, Output};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file_input: PathBuf,

    #[arg(short, long)]
    output_tcp: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.file_input.exists() {
        return Err(Error::Msg(format!(
            "file not found: {}",
            cli.file_input.to_string_lossy()
        )));
    }

    let input = FileSource::new(cli.file_input, None)?;
    let mut output = Console::new();

    if let Some(_tcp_addr) = cli.output_tcp {
        // TODO: a tcp output
    }

    for event in input {
        match event {
            Ok(event) => {
                output.emit(event)?;
            }
            Err(e) => {
                eprintln!("error reading event: {}", e);
            }
        }
    }

    Ok(())
}
