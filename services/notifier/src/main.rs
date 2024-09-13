use std::time::Duration;

use clokwerk::{AsyncScheduler, TimeUnits};
use libs::{common::config::get_config, repository::repository};

#[tokio::main]
async fn main() {
    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    scheduler
        .every(10.seconds())
        .run(|| async { check_modification().await });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

async fn check_modification() {
    let count = repository::get_amount().await;
    let last_value = get_last_value().await;

    if count > last_value {
        let current = count - last_value;
        notify(format!("You have {} new values in your database", current).as_str()).await;
        update_value(count).await;
    } else if count < last_value {
        let current = last_value - count;
        notify(format!("You have {} values removed from your database", current).as_str()).await;
        update_value(count).await;
    }
}

async fn get_last_value() -> i64 {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(get_config().unwrap().notifier.file);

    let count: i64 = if let Ok(file) = file {
        let count = serde_json::from_reader(file).unwrap_or_default();
        count
    } else {
        0
    };

    count
}

async fn update_value(value: i64) {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config().unwrap().notifier.file);

    if let Ok(file) = file {
        serde_json::to_writer_pretty(file, &value).unwrap();
    }
}

async fn notify(message: &str) {
    let response = ureq::post("https://api.pushbullet.com/v2/pushes")
        .set("Content-Type", "application/json; charset=utf-8")
        .set("Access-Token", &get_config().unwrap().notifier.token)
        .send_json(ureq::json!({
            "body" : message,
            "title" : "calculator",
            "type" : "note"
        }))
        .unwrap();

    if response.status() != 200 {
        println!(
            "Error Code: {} Message: {}",
            response.status(),
            response.status_text()
        );
    }
}
