use clap::Parser;

use crate::{cli::Cli, commands::new::NewCommand};

mod cli;
mod commands;

fn main() {
    let args = Cli::parse();

    match args.get_command() {
        cli::Command::New(new_args) => {
            NewCommand::new(new_args)
                .create_require_directories_and_files()
                .init_git();
        }
    }

    println!("Hello, world!");
}
