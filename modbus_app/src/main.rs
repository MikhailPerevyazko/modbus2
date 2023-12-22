use std::io::{ErrorKind, Write};

use crate::{
    cmd::get_path,
    config_manager::{
        channel_config::{ChannelTcp, Connect},
        Config,
    },
};

mod cmd;
mod config_manager;
mod modbus_manager;
mod task;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world!");
    let configs = Config::try_read_config_file(get_path())?;
    let modbus_tcp_config = ChannelTcp::from(configs.channel().to_owned());
    loop {
        let mut stream = match modbus_tcp_config.connect() {
            Ok(stream) => {
                println!(
                    "Установлено соединение с клиентом: {}",
                    modbus_tcp_config.url()
                );
                Some(stream)
            }
            Err(err) => {
                println!(
                    "Ошибка установки соединения с клиентом {}: {err}",
                    modbus_tcp_config.url()
                );
                None
            }
        };
        let buf = [0u8; 256];
        loop {
            match &stream {
                Some(tcp_stream) => match tcp_stream.take_error() {
                    Ok(err) => match err {
                        Some(err) => match err.kind() {
                            ErrorKind::BrokenPipe
                            | ErrorKind::ConnectionAborted
                            | ErrorKind::ConnectionRefused
                            | ErrorKind::ConnectionReset => {
                                println!("Ошибка в канале связи {err}");
                                stream = None;
                                break;
                            }
                            _ => {
                                println!("Ошибка в канале связи {err}");
                                continue;
                            }
                        },
                        None => println!("Нет ошибок в канале связи"),
                    },
                    Err(err) => {
                        println!(
                            "Ошибка получения ошибки из канала связи {},{err}",
                            modbus_tcp_config.url()
                        );
                        stream = None;
                        break;
                    }
                },
                None => break,
            }
            match stream {
                Some(ref mut tcp_stream) => match tcp_stream.write(&buf) {
                    Ok(size) => println!("Записано байт:{size}"),
                    Err(err) => {
                        println!("Ошибка записи:{err}");
                        break;
                    }
                },
                None => {}
            }
        }
        std::thread::sleep(modbus_tcp_config.timeout().to_owned());
    }
}
