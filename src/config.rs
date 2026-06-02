use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub program: Program,
}

#[derive(Deserialize)]
pub struct Program {
    pub name: String,
    pub version: String,
}
