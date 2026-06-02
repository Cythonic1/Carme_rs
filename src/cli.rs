use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

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
    Build(BuildArgs),
    Run(RunArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    #[arg(long, short, default_value = "debug", help = "debug or release")]
    pub build_type: String,
    #[arg(
        long,
        short,
        default_value_t = 4,
        help = "number of jobs when building"
    )]
    pub jobs: i32,
}

impl Default for BuildArgs {
    fn default() -> Self {
        Self {
            build_type: String::from("Debug"),
            jobs: 4,
        }
    }
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(
        long,
        short,
        default_value_t = false,
        help = "building and running all togather uses default build config"
    )]
    pub build: bool,
}

#[derive(Args)]
pub struct NewArgs {
    #[arg(
        long,
        short,
        default_value_t = true,
        help = "initialize cmake [default: true]"
    )]
    pub init_cmake: bool,
    #[arg(help = "project name")]
    pub project_name: PathBuf,
    #[arg(long, short, default_value = "3.14", help = "Cmake minimum version")]
    pub cmake_version: String,
}
