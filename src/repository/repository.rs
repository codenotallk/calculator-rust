use std::fs::OpenOptions;

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

use crate::domain::operation::{Operation, OperationBuilder};

#[derive(Serialize, Deserialize)]
struct OperationMapper {
    name: String,
    value_1: u32,
    value_2: u32,
    result: u32,
    create_at: u64,
}

impl From<Operation> for OperationMapper {
    fn from(operation: Operation) -> Self {
        Self {
            name: operation.name(),
            value_1: operation.value_1(),
            value_2: operation.value_2(),
            result: operation.result(),
            create_at: operation.create_at(),
        }
    }
}

impl Into<Operation> for OperationMapper {
    fn into(self) -> Operation {
        OperationBuilder::new()
            .with_operation(self.name)
            .with_value_1(self.value_1)
            .with_value_2(self.value_2)
            .with_result(self.result)
            .with_epoch(self.create_at)
            .build()
    }
}

pub fn save(operation: Operation) {
    let mapper = OperationMapper::from(operation);

    let file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("report.csv")
    {
        Ok(file) => file,
        Err(_) => todo!(),
    };

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(mapper).unwrap();

    writer.flush().unwrap();
}

pub fn get() -> Vec<Operation> {
    let mut operations = Vec::new();

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path("report.csv")
        .unwrap();

    for record in reader.deserialize() {
        let mapper: OperationMapper = record.unwrap();

        operations.push(mapper.into());
    }

    operations
}
