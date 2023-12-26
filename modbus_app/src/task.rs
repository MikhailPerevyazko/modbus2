use clap::parser::Values;
use rmodbus::{client::ModbusRequest, guess_response_frame_len, ErrorKind, ModbusProto};
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
    mreq: Option<ModbusRequest>,
}
#[derive(Clone)]
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

impl Into<ModbusProto> for ProtocolType {
    fn into(self) -> ModbusProto {
        match &self {
            ProtocolType::Tcp => ModbusProto::TcpUdp,
            ProtocolType::Uart => ModbusProto::Rtu,
        }
    }
}

impl Task {
    pub fn generate_request(&mut self) -> Result<Vec<u8>, rmodbus::ErrorKind> {
        let mut mreq = ModbusRequest::new(self.unit_id, self.protocol.to_owned().into());

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
        }
        self.mreq = Some(mreq);
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    #[test]
    fn tcp_read_coil() -> Result<(), ErrorKind> {
        let mut task = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadCoilStatus,
            start: 0,
            count: 2,
            data: vec![],
            mreq: None,
        };
        let result = task.generate_request()?;
        println!("request: {:?}", result);
        assert_eq!(
            &result,
            &[0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x01, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_input_status() -> Result<(), ErrorKind> {
        let mut task_two = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadInputStatus,
            start: 0,
            count: 2,
            data: vec![],
            mreq: None,
        };
        let result_two = task_two.generate_request()?;
        println!("request: {:?}", result_two);
        assert_eq!(
            &result_two,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_holding_registers() -> Result<(), ErrorKind> {
        let mut task_three = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadHoldingRegisters,
            start: 0,
            count: 2,
            data: vec![],
            mreq: None,
        };
        let result_three = task_three.generate_request()?;
        println!("request: {:?}", result_three);
        assert_eq!(
            &result_three,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_read_input_registers() -> Result<(), ErrorKind> {
        let mut task_four = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ReadInputRegisters,
            start: 0,
            count: 2,
            data: vec![],
            mreq: None,
        };
        let result_four = task_four.generate_request()?;
        println!("request: {:?}", result_four);
        assert_eq!(
            &result_four,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x04, 0, 0, 0, 0x02]
        );
        Ok(())
    }

    #[test]
    fn tcp_force_single_coil() -> Result<(), ErrorKind> {
        let mut task_five = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ForceSingleCoil,
            start: 1,
            count: 1,
            data: vec![1],
            mreq: None,
        };
        let result_five = task_five.generate_request()?;
        println!("request: {:?}", result_five);
        assert_eq!(
            &result_five,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0, 0x01, 0xFF, 0x00]
        );
        Ok(())
    }

    #[test]
    fn tcp_preset_single_register() -> Result<(), ErrorKind> {
        let mut task_six = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::PresetSingleRegister,
            start: 1,
            count: 1,
            data: vec![0x55FF],
            mreq: None,
        };
        let result_six = task_six.generate_request()?;
        println!("request: {:?}", result_six);
        assert_eq!(
            &result_six,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0, 0x01, 0x55, 0xFF]
        );
        Ok(())
    }

    #[test]
    fn tcp_preset_multiple_register() -> Result<(), ErrorKind> {
        let mut task_seven = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::PresetMultipleRegisters,
            start: 0,
            count: 2,
            data: vec![0x000A, 0x0102],
            mreq: None,
        };
        let result_seven = task_seven.generate_request()?;
        println!("request: {:?}", result_seven);
        assert_eq!(
            &result_seven,
            &[
                0, 0x01, 0x00, 0x00, 0x00, 0x0B, 0x01, 0x10, 0, 0x00, 0x00, 0x02, 0x04, 0x00, 0x0A,
                0x01, 0x02
            ]
        );
        Ok(())
    }

    #[test]
    fn tcp_force_multiple_coils() -> Result<(), ErrorKind> {
        let mut task_eight = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Tcp,
            command: CommandType::ForceMultipleCoils,
            start: 0,
            count: 2,
            data: vec![0, 1],
            mreq: None,
        };
        let result_eight = task_eight.generate_request()?;
        println!("request: {:?}", result_eight);
        assert_eq!(
            &result_eight,
            &[0, 0x01, 0x00, 0x00, 0x00, 0x08, 0x01, 0x0F, 0, 0x00, 0x00, 0x02, 0x01, 0x02]
        );
        Ok(())
    }
    #[test]
    fn rtu_read_coil() -> Result<(), ErrorKind> {
        let mut task_one_rtu = Task {
            id: 1,
            unit_id: 1,
            protocol: ProtocolType::Uart,
            command: CommandType::ReadCoilStatus,
            start: 19,
            count: 37,
            data: vec![],
            mreq: None,
        };
        let result_one_rtu = task_one_rtu.generate_request()?;
        println!("request: {:?}", result_one_rtu);
        assert_eq!(
            &result_one_rtu,
            &[0x01, 0x01, 0x00, 0x13, 0x00, 0x25, 0x0C, 0x14]
        );
        Ok(())
    }

    #[test]
    fn rtu_read_input_status() -> Result<(), ErrorKind> {
        let mut task_two_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::ReadInputStatus,
            start: 196,
            count: 22,
            data: vec![],
            mreq: None,
        };
        let result_two_rtu = task_two_rtu.generate_request()?;
        println!("request: {:?}", result_two_rtu);
        assert_eq!(
            &result_two_rtu,
            &[0x11, 0x02, 0x00, 0xC4, 0x00, 0x16, 0xBA, 0xA9]
        );
        Ok(())
    }

    #[test]
    fn rtu_read_holding_registers() -> Result<(), ErrorKind> {
        let mut task_three_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::ReadHoldingRegisters,
            start: 107,
            count: 3,
            data: vec![],
            mreq: None,
        };
        let result_three_rtu = task_three_rtu.generate_request()?;
        println!("request: {:?}", result_three_rtu);
        assert_eq!(
            &result_three_rtu,
            &[0x11, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87]
        );
        Ok(())
    }

    #[test]
    fn rtu_read_input_registers() -> Result<(), ErrorKind> {
        let mut task_four_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::ReadInputRegisters,
            start: 8,
            count: 1,
            data: vec![],
            mreq: None,
        };
        let result_four_rtu = task_four_rtu.generate_request()?;
        println!("request: {:?}", result_four_rtu);
        assert_eq!(
            &result_four_rtu,
            &[0x11, 0x04, 0x00, 0x08, 0x00, 0x01, 0xB2, 0x98]
        );
        Ok(())
    }

    #[test]
    fn rtu_force_single_coil() -> Result<(), ErrorKind> {
        let mut task_five_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::ForceSingleCoil,
            start: 172,
            count: 3,
            data: vec![1],
            mreq: None,
        };
        let result_five_rtu = task_five_rtu.generate_request()?;
        println!("request: {:?}", result_five_rtu);
        assert_eq!(
            &result_five_rtu,
            &[0x11, 0x05, 0x00, 0xAC, 0xFF, 0x00, 0x4E, 0x8B]
        );
        Ok(())
    }

    #[test]
    fn rtu_preset_single_register() -> Result<(), ErrorKind> {
        let mut task_six_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::PresetSingleRegister,
            start: 1,
            count: 0,
            data: vec![0x0003],
            mreq: None,
        };
        let result_six_rtu = task_six_rtu.generate_request()?;
        println!("request: {:?}", result_six_rtu);
        assert_eq!(
            &result_six_rtu,
            &[0x11, 0x06, 0x00, 0x01, 0x00, 0x03, 0x9A, 0x9B]
        );
        Ok(())
    }

    // #[test]
    // fn rtu_force_multiple_coils() -> Result<(), ErrorKind> {
    //     let mut task_seven_rtu = Task {
    //         id: 1,
    //         unit_id: 17,
    //         protocol: ProtocolType::Uart,
    //         command: CommandType::ForceMultipleCoils,
    //         start: 19,
    //         count: 10,
    //         data: vec![1,1,0,0,1,1,0,1,0,1],
    //         mreq: None,
    //     };
    //     let result_seven_rtu = task_seven_rtu.generate_request()?;
    //     println!("request: {:?}", result_seven_rtu);
    //     assert_eq!(
    //         &result_seven_rtu,
    //         &[0x11, 0x0F, 0x00, 0x13, 0x00, 0x0A, 0x02, 0xCD, 0x01, 0xBF, 0x0B]
    //     );
    //     Ok(())
    // }
    #[test]
    fn rtu_preset_multiple_registers() -> Result<(), ErrorKind> {
        let mut task_eight_rtu = Task {
            id: 1,
            unit_id: 17,
            protocol: ProtocolType::Uart,
            command: CommandType::PresetMultipleRegisters,
            start: 1,
            count: 2,
            data: vec![0x000A, 0x0102],
            mreq: None,
        };
        let result_eight_rtu = task_eight_rtu.generate_request()?;
        println!("request: {:?}", result_eight_rtu);
        assert_eq!(
            &result_eight_rtu,
            &[0x11, 0x10, 0x00, 0x01, 0x00, 0x02, 0x04, 0x00, 0x0A, 0x01, 0x02, 0xC6, 0xF0]
        );
        Ok(())
    }

    
}

impl Task {
    pub fn get_responce_len(&self, data: &[u8]) -> Result<u8, ErrorKind> {
        Ok(guess_response_frame_len(
            data,
            self.protocol.to_owned().into(),
        )?)
    }
}
