use anyhow::Result;
use clap::{Parser, Subcommand};
use pge_status::{from_file, request};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    StreetAddress { addr: String },
    TestFile { path: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let info = match &args.command {
        Commands::StreetAddress { addr } => request(addr).await?,
        Commands::TestFile { path } => from_file(path)?,
    };
    println!("{:#?}", info);
    Ok(())
}
