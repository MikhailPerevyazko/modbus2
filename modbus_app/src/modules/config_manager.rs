use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigItems {
    host: String,
    port: i64,
    pause: i64,
    var_name: String,
    storage_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    config: Vec<ConfigItems>,
}

pub fn read_file(state: tauri::State<PathState>) -> String {
    let file = std::fs::File::open(state.path()).unwrap();
    return file;
    modules::get_path(); 
}
