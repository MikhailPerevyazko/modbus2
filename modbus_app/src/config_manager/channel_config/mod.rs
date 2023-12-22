use std::{net::TcpStream, time::Duration};

use getset::Getters;
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
/// Структура описывает конфигурацию соединения с клиентом modbus
pub struct ChannelConfig {
    /// Адрес устройства
    pub host: Option<String>,
    /// порт устройства
    pub port: Option<u32>,
    /// UART device path
    //path: Option<String>,
    /// Настройка скорости приема передачи в бот
    pub baud_rate: Option<f64>,
    pub timeout: Option<f64>,
}

impl From<ChannelConfig> for ChannelTcp {
    fn from(value: ChannelConfig) -> Self {
        Self {
            host: match value.host {
                Some(host) => host,
                None => "127.0.0.1".to_string(),
            },
            port: match value.port {
                Some(port) => port,
                None => 502,
            },
            timeout: match value.timeout {
                Some(timeout) => Duration::from_secs_f64(timeout),
                None => Duration::from_millis(300),
            },
        }
    }
}
pub trait Connect {
    type Output;
    fn connect(&self) -> Self::Output;
}
#[derive(Getters)]
#[get = "pub"]
pub struct ChannelTcp {
    host: String,
    port: u32,
    timeout: Duration,
}

impl ChannelTcp {
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Connect for ChannelTcp {
    type Output = Result<TcpStream, Box<dyn std::error::Error>>;
    fn connect(&self) -> Self::Output {
        let mut stream = TcpStream::connect(self.url())?;
        stream.set_nonblocking(true);
        stream.set_read_timeout(Some(self.timeout.clone()))?;
        stream.set_write_timeout(Some(self.timeout.clone()))?;
        Ok(stream)
    }
}
