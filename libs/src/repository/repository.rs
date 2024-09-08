// use std::fs::OpenOptions;

// use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use tokio_postgres::{connect, NoTls};

use crate::domain::operation::{Operation, OperationBuilder};

#[derive(Serialize, Deserialize)]
struct OperationMapper {
    name: String,
    value_1: i32,
    value_2: i32,
    result: i32,
    create_at: i64,
}

impl From<Operation> for OperationMapper {
    fn from(operation: Operation) -> Self {
        Self {
            name: operation.name(),
            value_1: operation.value_1() as i32,
            value_2: operation.value_2() as i32,
            result: operation.result() as i32,
            create_at: operation.create_at() as i64,
        }
    }
}

impl Into<Operation> for OperationMapper {
    fn into(self) -> Operation {
        OperationBuilder::new()
            .with_operation(self.name)
            .with_value_1(self.value_1 as u32)
            .with_value_2(self.value_2 as u32)
            .with_result(self.result as u32)
            .with_epoch(self.create_at as u64)
            .build()
    }
}

pub struct Interval {
    offset: Option<u32>,
    from: Option<i64>,
    to: Option<i64>,
}

impl Interval {
    pub fn new(offset: Option<u32>, from: Option<i64>, to: Option<i64>) -> Self {
        Self { offset, from, to }
    }
}

pub async fn save(operation: Operation) {
    let mapper = OperationMapper::from(operation);

    let url = "host=localhost port=5432 user=root password=root dbname=report_db";

    let (client, connection) = connect(url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    client.query("insert into reports_tb (operation, value_1, value_2, result, create_at) values ($1, $2, $3, $4, $5)",
     &[
        &mapper.name,
        &mapper.value_1,
        &mapper.value_2,
        &mapper.result,
        &mapper.create_at
     ]).await.unwrap();

    // let file = match OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("report.csv")
    // {
    //     Ok(file) => file,
    //     Err(_) => todo!(),
    // };

    // let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    // writer.serialize(mapper).unwrap();

    // writer.flush().unwrap();
}

pub async fn get(interval: Interval) -> Vec<Operation> {
    let mut operations = Vec::new();

    let url = "host=localhost port=5432 user=root password=root dbname=report_db";

    let (client, connection) = connect(url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    let query = query_build(interval);

    for row in client.query(query.as_str(), &[]).await.unwrap() {
        let mapper = OperationMapper {
            name: row.get(1),
            value_1: row.get(2),
            value_2: row.get(3),
            result: row.get(4),
            create_at: row.get(5),
        };

        operations.push(mapper.into());
    }
    // let mut reader = ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_path("report.csv")
    //     .unwrap();

    // for record in reader.deserialize() {
    //     let mapper: OperationMapper = record.unwrap();

    //     operations.push(mapper.into());
    // }

    operations
}

fn query_build(interval: Interval) -> String {
    let mut query = "select * from reports_tb ".to_string();

    let offset = match interval.offset {
        Some(offset) => offset,
        None => 0,
    };

    match (interval.from, interval.to) {
        (Some(from), Some(to)) => {
            query.push_str(&format!("where create_at between {} and {} ", from, to));
        }
        (Some(from), _) => {
            query.push_str(&format!("where create_at >= {} ", from));
        }
        (_, Some(to)) => {
            query.push_str(&format!("where create_at <= {} ", to));
        }

        _ => (),
    }

    query.push_str(&format!("order by id desc limit 20 offset {}", offset));

    query
}
