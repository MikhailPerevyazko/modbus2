use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

//pub use crate::Config::try_read_config_file;

pub fn client() {
    let timeout = Duration::from_secs(1);
    //Подключение к локальному хосту
    let mut stream = TcpStream::connect("127.0.0.1").unwrap();
    stream.set_read_timeout(Some(timeout)).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();
    //Содаем пустой запрос modbus
    let mut mreq = ModbusRequest::new(1, ModbusProto::TcpUdp);
    mreq.tr_id = 2;
    //Создание адреса 8, 9 и запись в request
    let mut request = Vec::new();
    mreq.generate_set_coils_bulk(8, &[true, true], &mut request)
        .unwrap();
    //Отправка request
    stream.write(&request).unwrap();
    //Создание массива из первых шести элементов равных нулю типа u8
    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();
    //Формирует буфер ответа
    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    let len = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if len > 6 {
        let mut rest = vec![0u8; (len - 6) as usize];
        stream.read_exact(&mut rest).unwrap();
        response.extend(rest);
    }

    mreq.parse_ok(&response).unwrap();

    mreq.generate_get_coils(0, 5, &mut request).unwrap();
    stream.write(&request).unwrap();
    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();
    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    let len = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if len > 6 {
        let mut rest = vec![0u8; (len - 6) as usize];
        stream.read_exact(&mut rest).unwrap();
        response.extend(rest);
    }

    let mut data = Vec::new();
    mreq.parse_bool(&response, &mut data).unwrap();
    for i in 0..data.len() {
        println!("{} {}", i, data[i]);
    }
}
