use crate::{cmd::get_path, config_manager::Config};

mod cmd;
mod config_manager;
mod modbus_manager;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world!");
    let configs = Config::try_read_config_file(get_path())?;
    Ok(())
}
