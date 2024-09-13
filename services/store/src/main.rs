use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::{Consumer, MessageSets},
};
use libs::{
    common::{config::get_config, time::get_epoch_from_formatted},
    domain::operation::{Operation, OperationBuilder},
    repository::repository,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalculateRequest {
    operation: String,
    value_1: u32,
    value_2: u32,
    result: u32,
    create_at: String,
}

impl Into<Operation> for CalculateRequest {
    fn into(self) -> Operation {
        OperationBuilder::new()
            .with_operation(self.operation)
            .with_value_1(self.value_1)
            .with_value_2(self.value_2)
            .with_result(self.result)
            .with_epoch(get_epoch_from_formatted(&self.create_at).unwrap() as u64)
            .build()
    }
}

#[tokio::main]
async fn main() {
    let config = get_config().unwrap();

    let mut consumer = Consumer::from_hosts(config.broker.hosts)
        .with_topic(config.store.topic)
        .with_group(config.store.group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();

    loop {
        match consumer.poll() {
            Ok(mss) => {
                if !mss.is_empty() {
                    process_records(mss).await;
                }
            }
            Err(_) => todo!(),
        }
    }
}

async fn process_records(mss: MessageSets) {
    for ms in mss.iter() {
        for m in ms.messages() {
            let request: CalculateRequest = serde_json::from_slice(&m.value).unwrap();
            repository::save(request.into()).await;
        }
    }
}
