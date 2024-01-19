use serde::Deserialize;

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

#[derive(Debug, Deserialize, Clone)]
//Структура описывает конфигурацию modbus запроса
pub struct ConfigItem {
    pub storage: String,
    pub id: u16,
    pub unit_id: u8,
    pub name: String,
    pub start: u16,
}

impl From<ConfigItem> for ModbusRequestItems {
    fn from(value: ConfigItem) -> Self {
        Self {
            storage: ModbusStorage::from(value.storage.to_owned()),
            id: value.id,
            unit_id: value.unit_id,
            name: value.name,
            start: value.start,
        }
    }
}

pub struct ModbusRequestItems {
    pub storage: ModbusStorage,
    pub id: u16,
    pub unit_id: u8,
    pub name: String,
    pub start: u16,
}
