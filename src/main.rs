#![deny(unsafe_code)]
mod version;

use clap::Parser;
use log::info;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing command line arguments")]
    CliParser(#[from] clap::Error),
}

#[derive(Parser, Debug)]
#[clap(version = version::VERSION.as_str(), term_width=0)]
struct Args {
    /// DELETEME: example argument
    #[clap(short, long, default_value = "Hello world!")]
    arg: String,
}

fn main() -> Result<(), Error> {
    // Example of configuring env_logger:
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line arguments
    let args: Args = Args::try_parse().map_err(Error::CliParser)?;

    info!("Running jd-test-repo version {}", version::VERSION.as_str());

    // DELETEME: Your code here...
    println!("Arg = {}", args.arg);

    Ok(())
}
