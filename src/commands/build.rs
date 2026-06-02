use std::process::Command;

use log::{info, warn};

use crate::cli::BuildArgs;

pub struct BuildCommand {
    build_args: BuildArgs,
}

impl BuildCommand {
    pub fn new(build_args: BuildArgs) -> Self {
        Self { build_args }
    }

    // execlp("cmake", "cmake", "-S", project_root_path.c_str(), "-B", project_build_path.c_str(),
    //        build_type.c_str(), nullptr);
    //execlp("cmake", "cmake", "--build", project_build_path.c_str(), nullptr);

    pub fn run(&self) {
        let root_project_path =
            std::env::current_dir().expect("unable to get the current directory");
        let build_project_path = root_project_path.join("build");
        let configs = Command::new("cmake")
            .arg("-S")
            .arg(root_project_path)
            .arg("-B")
            .arg(&build_project_path)
            .spawn();

        match configs {
            Ok(mut child) => match child.wait() {
                Ok(status) if status.success() => {
                    info!("Cmake has been config building now");
                }
                Ok(status) => {
                    warn!("CMake build exited with status: {}", status);
                }
                Err(err) => {
                    warn!("Failed to wait on CMake build: {}", err);
                }
            },
            Err(err) => {
                warn!("Unable to applie Cmake configs: {}", err);
            }
        }

        let build = Command::new("cmake")
            .arg("--build")
            .arg(&build_project_path)
            .arg("-j")
            .arg(self.build_args.jobs.to_string())
            .spawn();

        match build {
            Ok(mut child) => match child.wait() {
                Ok(status) if status.success() => {
                    info!("project has been build");
                }
                Ok(status) => {
                    warn!("Error building project: exit code {}", status);
                }
                Err(err) => {
                    warn!("Error building project: {}", err);
                }
            },
            Err(err) => {
                warn!("Unable to applie Cmake configs: {}", err);
            }
        }
    }
}
