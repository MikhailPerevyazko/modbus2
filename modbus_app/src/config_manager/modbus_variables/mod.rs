use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModbusConfigVariable {
    storage: String,
}

pub enum ModbusStorage {
    DI,
    DO,
    AI,
    AO,
}

impl From<String> for ModbusStorage {
    fn from(value: String) -> Self {
        match &value.to_lowercase()[..] {
            "di" => ModbusStorage::DI,
            "do" => ModbusStorage::DO,
            "ai" => ModbusStorage::AI,
            "ao" => ModbusStorage::AO,
            _ => ModbusStorage::AI,
        }
    }
}
