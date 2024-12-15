use std::fs;
use log::LevelFilter;
use mycoforge::dataset::logger::{SimpleLogger, PostgresLogger};

#[test]
fn test_file_logging() {

    let test_file = "test.log";
    let logger = SimpleLogger::new(Some(test_file.to_string()), LevelFilter::Debug);
    log::set_boxed_logger(Box::new(logger)).expect("Failed to set boxed logger!");
    log::set_max_level(LevelFilter::Debug);

    log::info!("test message");

    let contents = fs::read_to_string(test_file).unwrap_or_else(|_| panic!("Failed to load contents from {}", test_file));
    assert!(contents.contains("test message"));

    fs::remove_file(test_file).unwrap_or_else(|_| panic!("Failed to delete file {}", test_file));
}

#[tokio::test]
async fn test_postgres_logging() {
    let logger = PostgresLogger::new(LevelFilter::Debug).expect("Failed to create PostgresLogger");
    let logger_ref = &logger.clone();
    log::set_boxed_logger(Box::new(logger)).expect("Failed to set boxed logger!");
    log::set_max_level(LevelFilter::Debug);

    log::info!("test message");

    if let Ok(client) = logger_ref.db_client().lock() {
        let rows = client.query(
        "select message from logs where message like '%test message%'", &[])
            .await.expect("Failed to retrieve test message!");
        assert!(!rows.is_empty(), "Extracted message is empty!");
    };
}

