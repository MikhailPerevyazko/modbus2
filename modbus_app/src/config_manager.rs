use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml;

mod cmd::get_path();

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigItems {
    pub host: String,
    pub port: i64,
    pub pause: i64,
    pub var_name: String,
    pub storage_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub config: Vec<ConfigItems>,
}

pub use crate::cmd::get_path;

pub struct PathState(PathBuf);
    impl PathState {
        pub fn path(&self) -> PathBuf {
            cmd::get_path();
            self.0.to_owned()
        }
    }
    

pub fn file_open(state:PathState) -> String {
    let open_file = std::fs::File::open(state.path()).unwrap();
    let string_file = serde_yaml::from_reader(open_file).unwrap();
    let file = to_string_pretty(&string_file).unwarp();
    file
}