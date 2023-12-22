pub mod channel_config;
mod modbus_variables;
use getset::Getters;
use serde::Deserialize;

use std::path::PathBuf;

use self::{channel_config::ChannelConfig, modbus_variables::ModbusConfigVariable};

#[derive(Debug, Deserialize, Getters)]
#[get = "pub"]
pub struct Config {
    channel: ChannelConfig,
    variables: Vec<ModbusConfigVariable>,
}

impl Config {
    pub fn try_read_config_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let open_file = std::fs::File::open(path)?;
        let config: Self = serde_yaml::from_reader(open_file)?;
        println!("{:#?}", config.channel.host);
        Ok(config)
    }
}
