use std::{fs, io::Write, process::Command};

use crate::cli::NewArgs;

pub struct NewCommand {
    args: NewArgs,
}

impl NewCommand {
    pub fn new(args: NewArgs) -> Self {
        Self { args }
    }

    pub fn create_require_directories_and_files(&self) -> &Self {
        match fs::create_dir(&self.args.project_name) {
            Ok(_) => {
                println!("home dir has been created");
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }

        let src_path = self.args.project_name.join("src");
        fs::create_dir(&src_path).expect("Unable to create the src directory");

        let mut src_file_handler =
            fs::File::create(src_path.join("main.cc")).expect("Unable to create the main.cc file");

        src_file_handler
            .write_all(NewCommand::default_main().as_bytes())
            .expect("Unable to write the src of main.cc to it");

        let mut cmake_file_handler =
            fs::File::create(self.args.project_name.join("CMakeLists.txt"))
                .expect("Unable to create CMakeLists file");
        cmake_file_handler
            .write_all(
                NewCommand::default_cmake(
                    &self.args.project_name.to_string_lossy(),
                    &self.args.cmake_version,
                )
                .as_bytes(),
            )
            .expect("Unable to write cmake default content");

        fs::create_dir(self.args.project_name.join("build"))
            .expect("Unable to create the src directory");
        fs::create_dir(self.args.project_name.join("includes"))
            .expect("Unable to create the src directory");

        self
    }

    pub fn init_git(&self) {
        let command = Command::new("git")
            .arg("init")
            .arg(&self.args.project_name)
            .spawn();

        match command {
            Ok(mut child_process) => match child_process.wait() {
                Ok(_) => {
                    println!("git has been initialized");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    fn default_main() -> &'static str {
        r#"
#include <print>
int main(int argc, char *argv[]) {
    std::println("Hello World");
    return 0;
}"#
    }

    fn default_cmake(project_name: &str, cmake_version: &str) -> String {
        format!(
            r#"cmake_minimum_required(VERSION {cmake_version})
project({project_name})

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_compile_options(-Wall -Wextra -Wpedantic)
file(GLOB_RECURSE SOURCE "src/*.cc")

add_executable({project_name} ${{SOURCE}})
"#
        )
    }
}
