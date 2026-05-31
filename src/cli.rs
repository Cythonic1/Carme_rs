use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn get_command(self) -> Command {
        self.command
    }
}

#[derive(Subcommand)]
pub enum Command {
    New(NewArgs),
}

#[derive(Args)]
pub struct NewArgs {
    #[arg(long, default_value = ".", help = "Working directory")]
    pub home: PathBuf,
    #[arg(
        long,
        short,
        default_value_t = true,
        help = "initialize cmake [default: true]"
    )]
    pub init_cmake: bool,
    #[arg(long, short, default_value = "MyApp", help = "project name")]
    pub project_name: String,
    #[arg(long, short, default_value = "3.14", help = "Cmake minimum version")]
    pub cmake_version: String,
}
