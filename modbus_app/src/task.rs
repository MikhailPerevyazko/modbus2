use rmodbus::{client::ModbusRequest, ErrorKind, ModbusProto};

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
                    mreq.generate_set_holding(self.start, data.to_owned(), &mut request)?
                }
            },
            CommandType
            _ => {}
        }

        Ok((request, mreq))
    }
}
