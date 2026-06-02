use clap::Parser;

use crate::{
    cli::{BuildArgs, Cli},
    commands::{build::BuildCommand, new::NewCommand, run::RunCommand},
};

mod cli;
mod commands;
mod config;

fn main() {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();

    match args.get_command() {
        cli::Command::New(new_args) => {
            NewCommand::new(new_args)
                .create_require_directories_and_files()
                .init_git();
        }
        cli::Command::Build(build_args) => {
            BuildCommand::new(build_args).run();
        }
        cli::Command::Run(run_args) => {
            RunCommand::new(run_args).unwrap().run();
        }
    }
}
