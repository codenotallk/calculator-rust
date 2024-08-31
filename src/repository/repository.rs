use std::fs::OpenOptions;

use csv::WriterBuilder;
use serde::Serialize;

use crate::domain::operation::Operation;


#[derive(Serialize)]
struct OperationMapper {
    name: String,
    value_1: u32,
    value_2: u32,
    result: u32,
}

impl From<Operation> for OperationMapper {
    fn from(operation: Operation) -> Self {
        Self {
            name: operation.name(),
            value_1: operation.value_1(),
            value_2: operation.value_2(),
            result: operation.result()
        }
    }
}

pub fn save (operation: Operation) {
    let mapper = OperationMapper::from(operation);

    let file = match OpenOptions::new().create(true).append(true).open("operations.csv") {
        Ok(file) => file,
        Err(_) => todo!(),
    };

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(mapper).unwrap();

    writer.flush().unwrap();
}