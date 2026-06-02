use std::{env, fs, process::Command};

use log::{info, warn};

use crate::{
    cli::{BuildArgs, RunArgs},
    commands::{DEFAULT_CONFIG_PATH, build::BuildCommand},
    config::Config,
};

pub struct RunCommand {
    run_args: RunArgs,
    config: Config,
}

impl RunCommand {
    pub fn new(run_args: RunArgs) -> Option<Self> {
        let config_path = env::current_dir()
            .expect("unable to get the current directory")
            .join(DEFAULT_CONFIG_PATH);

        match fs::exists(&config_path) {
            Ok(does_exists) => {
                if !does_exists {
                    warn!("unable to read Carme.toml file does not exist");
                    return None;
                }
            }
            Err(err) => {
                warn!("{err}");
                return None;
            }
        }
        let config_file: Config = toml::from_str(
            &std::fs::read_to_string(config_path).expect("unable to open Carme.toml"),
        )
        .expect("error reading Carme.toml");

        Some(Self {
            run_args,
            config: config_file,
        })
    }

    pub fn run(&self) {
        let root_path = env::current_dir().expect("unable to get the cwd");
        let program_name = root_path.join("build").join(&self.config.program.name);

        if self.run_args.build {
            BuildCommand::new(BuildArgs::default()).run();
        }

        let program = Command::new(program_name).spawn();

        match program {
            Ok(mut child) => match child.wait() {
                Ok(status) if status.success() => {
                    info!("Finish executing program");
                }
                Ok(status) => {
                    warn!("program exit with bad status code: {}", status);
                }
                Err(err) => {
                    warn!("Failed to wait on prorgam execution: {}", err);
                }
            },
            Err(err) => {
                warn!("Unable to execute the program: {}", err);
            }
        }
    }
}
