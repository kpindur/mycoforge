use std::fs;
use log::LevelFilter;
use mycoforge::dataset::logger::SimpleLogger;

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
