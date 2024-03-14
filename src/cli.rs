use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct ZonedCli {
    #[clap(subcommand)]
    pub mode: Mode,
}

impl ZonedCli {
    pub fn do_parse() -> Self {
        ZonedCli::parse()
    }
}

#[derive(Subcommand)]
pub enum Mode {
    /// Converts ZoneData to PlainText
    Convert(File),
    /// Todo: Compiles Plaintext into ZoneData
    Compile(File),
}

#[derive(Parser)]
#[clap(about)]
pub struct File {
    #[clap(subcommand)]
    pub file: ZoneDataFile,
}

#[derive(Parser)]
pub enum ZoneDataFile {
    ZoneHeader(InputOutput),
    ZoneEntities(InputOutput),
}

#[derive(Parser)]
pub struct InputOutput {
    #[arg(short, value_name = "INPUT")]
    pub input: PathBuf,

    #[arg(short, value_name = "PATH")]
    pub output: Option<String>,
}
