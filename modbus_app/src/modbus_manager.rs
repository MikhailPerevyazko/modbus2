use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};


pub fn client() {
    let timeout = Duration::from_secs(1);

    let mut stream = TcpStream::connect("127.0.0.1:5500").unwrap();
    stream.set_read_timeout(Some(timeout)).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();

    let mut mreq = ModbusRequest::new(1, ModbusProto::TcpUdp);
    mreq.tr_id = 2;

    let mut request = Vec::new();
    mreq.generate_set_coils_bulk(8, &[true, true], &mut request)
    .unwrap();

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