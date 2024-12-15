use std::fs;
use log::LevelFilter;
use mycoforge::dataset::logger::LogEntries;
use mycoforge::dataset::logger::Logger;
use mycoforge::dataset::logger::SimpleLogger;

use std::sync::{Arc, Mutex};

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

use mycoforge::dataset::logger::PostgresLogger;
use std::thread;
use std::time::Duration;
use log::{Level, Record, Log};

#[test]
fn test_postgres() {
    let entries = Arc::new(Mutex::new(LogEntries::default()));
    let logger = Logger::new(entries.clone(), LevelFilter::Debug);
    let db = PostgresLogger::new("test", entries.clone()).expect("Failed to connect to Postgresql!");

    thread::spawn(move || {
        if let Err(e) = db.run(Duration::from_secs(1)) { eprintln!("DB error in test: {}", e) }
    });

    logger.log(&Record::builder()
        .args(format_args!("Test message 1"))
        .level(Level::Info)
        .target("test")
        .file(Some(file!()))
        .line(Some(line!()))
        .module_path(Some(module_path!()))
        .build()
    );

    logger.log(&Record::builder()
        .args(format_args!("Test message 2"))
        .level(Level::Warn)
        .target("test")
        .file(Some(file!()))
        .line(Some(line!()))
        .module_path(Some(module_path!()))
        .build()
    );

    thread::sleep(Duration::from_secs(2));

    let mut client = PostgresLogger::new("test", entries.clone()).expect("Failed to connect to Postgresql!");

    let rows = client.db_client_mut().query(
        "select * from test order by timestamp", &[]
    ).expect("Successfully retrieved rows from database!");

    println!("Retrieved {} rows!", rows.len());
    assert_eq!(rows.len(), 2,
        "Retreived more rows than should have! Expected {}, found {}", 2, rows.len()
    );
    client.db_client_mut().query("drop table if exists test", &[]).expect("Successfully dropped table: test");
}
