use log::{Log, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SimpleLogger {
    file_path: Option<String>,
    console_level: LevelFilter
}

impl SimpleLogger {
    pub fn new(file_path: Option<String>, console_level: LevelFilter) -> Self { return Self { file_path, console_level }; }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        return metadata.level() <= self.console_level;
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get current time!")
            .as_secs();

        let log_entry = format!(
            "[{}] {} - {}\n",
            timestamp, record.level(), record.args()
        );

        println!("{}", log_entry);

        if let Some(path) = &self.file_path {
            if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(path) {
                let _ = file.write_all(log_entry.as_bytes());
            }
        }
    }

    fn flush(&self) {}
}

use std::env::var;
use std::error::Error;
use postgres::{Client, NoTls};
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::time::Duration;

pub struct LogEntries {
    entries: Vec<String>
}

impl Default for LogEntries {
    fn default() -> Self { return Self::new(Vec::new()); }
}

impl LogEntries {
    pub fn new(entries: Vec<String>) -> Self { return Self { entries }; }

    pub fn add(&mut self, entry: String) { self.entries.push(entry); }

    pub fn take_all(&mut self) -> Vec<String> { return std::mem::take(&mut self.entries); }
}

pub struct Logger {
    entries: Arc<Mutex<LogEntries>>,
    control_level: LevelFilter,
}

impl Logger {
    pub fn new(entries: Arc<Mutex<LogEntries>>, control_level: LevelFilter) -> Self { 
        return Self { entries, control_level };
    }

    fn create_timestamp() -> String {
        let output = Command::new("date")
            .arg("+%Y-%m-%d %H:%M:%S")
            .output()
            .expect("Failed to fetch date via bash!");

        return String::from_utf8(output.stdout)
            .expect("Failed to convert output to String!")
            .trim().to_string();
    }

    fn format_log(&self, record: &Record) -> String {
        let timestamp = Self::create_timestamp();
        let level = record.level().to_string();
        let target = record.target().to_string();
        let message = record.args().to_string();
        let module_path = record.module_path().unwrap_or(&String::default()).to_string();
        let file = record.file().unwrap_or(&String::default()).to_string();
        let line = record.line().map(|l| l.to_string()).unwrap_or_default();
        return format!(
            "{},{},{},{},{},{},{}",
            timestamp, level, target, message, module_path, file, line
        );
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool { return metadata.level() <= self.control_level; }
    fn flush(&self) {}

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        let formatted = self.format_log(record);
        if let Ok(mut entries) = self.entries.lock() {
            entries.add(formatted);
        }
    }
}

pub struct PostgresLogger {
    db_client: Client,
    tablename: String,
    entries: Arc<Mutex<LogEntries>>
}

impl PostgresLogger {
    pub fn new(tablename: &str, entries: Arc<Mutex<LogEntries>>) -> Result<Self, Box<dyn Error>> {
        let username = var("POSTGRES_USER").expect("Failed to fetch the username!");
        let password = var("POSTGRES_PASSWORD").expect("Failed to fetch the password!");
        let hostname = var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let dbname = var("POSTGRES_DB").unwrap_or_else(|_| "logs".to_string());

        let mut db_client = Client::connect(
            &format!("postgresql://{}:{}@{}/{}", username, password, hostname, dbname), NoTls)
            .expect("Failed to connect to postgresql!");
        {
            let query = format!("select exists ( select from pg_tables where tablename = '{}' )", tablename);

            let exists = db_client.transaction()
            .expect("Could not create transaction!")
            .query_one(&query, &[])?;

            if !exists.get::<_, bool>(0) {
                let query = format!("
                    create table {} (
                        id serial primary key,
                        timestamp timestamp,
                        level text,
                        target text,
                        message text,
                        module_path text,
                        file text,
                        line integer
                    )
                ", tablename);
                let mut transaction = db_client.transaction().expect("Could not create transaction!");
                transaction.execute(&query, &[])
                    .unwrap_or_else(|_| panic!("Failed to create table {}, even though it does not exist!", tablename));
                transaction.commit().expect("Failed to commit table creation!");
            }

        }
        let tablename = tablename.to_string();
        return Ok(Self { db_client, tablename, entries});
    }
    
    pub fn db_client(&self) -> &Client { return &self.db_client; }
    pub fn db_client_mut(&mut self) -> &mut Client { return &mut self.db_client; }

    pub fn run(mut self, interval: Duration) -> Result<(), Box<dyn Error>> {
        loop {
            thread::sleep(interval);
            self.flush()?;
        }
    }

    fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        let entries = if let Ok(mut entries) = self.entries.lock() {
            entries.take_all()
        } else {
            return Ok(());
        };

        if entries.is_empty() { return Ok(()); }

        let mut transaction = self.db_client.transaction()?;

        let statement = format!("insert into {} (timestamp, level, target, message, module_path, file, line)
            values (to_timestamp($1, 'YYYY-MM-DD HH24:MI:SS'),$2,$3,$4,$5,$6,$7)", self.tablename);
        let query = transaction.prepare(&statement)?;

        for entry in entries {
            let parts: Vec<&str> = entry.split(',').collect::<Vec<&str>>();
            if parts.len() != 7 { break; }

            let _ = transaction.execute(&query, 
            &[&parts[0], &parts[1], &parts[2], &parts[3], &parts[4], &parts[5], &parts[6].parse::<i32>().expect("Unparseable value!")]
            );
        }
        transaction.commit()?;

        return Ok(());
    }
}

