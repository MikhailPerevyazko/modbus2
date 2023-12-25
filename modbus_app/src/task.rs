use clap::parser::Values;
use rmodbus::{client::ModbusRequest, ErrorKind, ModbusProto};
use serde::de::value;
//use serde::de::value;

pub struct Task {
    id: u16,
    unit_id: u8,
    protocol: ProtocolType,
    command: CommandType,
    start: u16,
    count: u16,
    data: Vec<u16>,
}

enum ProtocolType {
    Tcp,
    Uart,
}

enum CommandType {
    ReadCoilStatus,
    ReadInputStatus,
    ReadHoldingRegisters,
    ReadInputRegisters,
    ForceSingleCoil,
    PresetSingleRegister,
    ForceMultipleCoils,
    PresetMultipleRegisters,
}

impl Task {
    pub fn generate_request(&self) -> Result<(Vec<u8>, ModbusRequest), rmodbus::ErrorKind> {
        let mut mreq = match &self.protocol {
            ProtocolType::Tcp => ModbusRequest::new(self.unit_id, ModbusProto::TcpUdp),
            ProtocolType::Uart => ModbusRequest::new(self.unit_id, ModbusProto::Rtu),
        };
        mreq.tr_id = self.id;
        let mut request = Vec::new();
        match &self.command {
            CommandType::ReadCoilStatus => {
                mreq.generate_get_coils(self.start, self.count, &mut request)?;
            }
            CommandType::ReadInputStatus => {
                mreq.generate_get_discretes(self.start, self.count, &mut request)?;
            }
            CommandType::ReadHoldingRegisters => {
                mreq.generate_get_holdings(self.start, self.count, &mut request)?;
            }
            CommandType::ReadInputRegisters => {
                mreq.generate_get_inputs(self.start, self.count, &mut request)?;
            }
            CommandType::ForceSingleCoil => match self.data.get(0) {
                Some(data) => mreq.generate_set_coil(
                    self.start,
                    if data == &0 { false } else { true },
                    &mut request,
                )?,
                None => Err(ErrorKind::IllegalDataValue)?,
            },
            CommandType::PresetSingleRegister => match self.data.get(0) {
                Some(data) => {
                    mreq.generate_set_holding(self.start, data.to_owned(), &mut request)?;
                }
                None => Err(ErrorKind::CommunicationError)?,
            },
            CommandType::ForceMultipleCoils => {
                if !self.data.is_empty() {
                    mreq.generate_set_coils_bulk(
                        self.start,
                        &self.data.iter().map(|&x| x != 0).collect::<Vec<bool>>(),
                        &mut request,
                    )?;
                } else {
                    return Err(ErrorKind::Acknowledge)?;
                }
            }
            CommandType::PresetMultipleRegisters => {
                if !self.data.is_empty() {
                    mreq.generate_set_holdings_bulk(self.start, &self.data, &mut request)?;
                } else {
                    return Err(ErrorKind::Acknowledge)?;
                }
            }
            _ => {}
        }
        Ok((request, mreq))
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    #[test]
    fn tcp_read_coil() -> Result<(), ErrorKind> {
        let task = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadCoilStatus,
            start: 0,
            count: 2,
            data: vec![],
        };
        let result = task.generate_request()?;
        println!("request: {:?}", result.0);
        assert_eq!(
            &result.0,
            &[0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x01, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_input_status() -> Result<(), ErrorKind> {
        let task_two = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadInputStatus,
            start: 0,
            count: 2,
            data: vec![],
        };
        let result_two: (Vec<u8>, ModbusRequest) = task_two.generate_request()?;
        println!("request: {:?}", result_two.0);
        assert_eq!(
            &result_two.0,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_holding_registers() -> Result<(), ErrorKind> {
        let task_three = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadHoldingRegisters,
            start: 0,
            count: 2,
            data: vec![],
        };
        let result_three: (Vec<u8>, ModbusRequest) = task_three.generate_request()?;
        println!("request: {:?}", result_three.0);
        assert_eq!(
            &result_three.0,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_input_registers() -> Result<(), ErrorKind> {
        let task_four = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadInputRegisters,
            start: 0,
            count: 2,
            data: vec![],
        };
        let result_four: (Vec<u8>, ModbusRequest) = task_four.generate_request()?;
        println!("request: {:?}", result_four.0);
        assert_eq!(
            &result_four.0,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x04, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_force_single_coil() -> Result<(), ErrorKind> {
        let task_five = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ForceSingleCoil,
            start: 0,
            count: 2,
            data: vec![],
        };
        let result_five: (Vec<u8>, ModbusRequest) = task_five.generate_request()?;
        println!("request: {:?}", result_five.0);
        assert_eq!(
            &result_five.0,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0, 0x01, 0x11, 0x00]
        );
        Ok(())
    }
}
