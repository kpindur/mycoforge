use log::{Log, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
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

use std::error::Error;
use std::env;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls};

#[derive(Clone)]
pub struct PostgresLogger {
    db_client: Arc<Mutex<Client>>,
    console_level: LevelFilter,
}

impl PostgresLogger {
    pub fn new(console_level: LevelFilter) -> Result<Self, Box<dyn Error>> {
        let db_url = Self::construct_db_url()?;
        let rt = tokio::runtime::Runtime::new()?;

        let (db_client, connection) = rt.block_on(async {
            tokio_postgres::connect(&db_url, NoTls).await
        })?;

        rt.spawn(async move {
            if let Err(error) = connection.await { eprintln!("Database connection error: {}", error); }
        });

        rt.block_on(async {
            db_client.batch_execute("
                create table if not exists logs (
                    id serial primary key,
                    timestamp bigint,
                    level text,
                    target text,
                    message text,
                    module_path text,
                    file text,
                    line integer
                )
            ").await
        })?;

        let db_client = Arc::new(Mutex::new(db_client));

        return Ok(Self { db_client, console_level });
    }

    pub fn db_client(&self) -> &Mutex<Client> { return &self.db_client; }

    fn construct_db_url() -> Result<String, env::VarError> {
        let user = env::var("POSTGRES_USER")?;
        let password = env::var("POSTGRES_PASSWORD")?;
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or_else(|_| "logs".to_string());
        
        return Ok(format!(
            "postgresql://{}:{}@{}/{}",
            user, password, host, db
        ));
    }
}

impl Log for PostgresLogger {
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

        if let Ok(client) = self.db_client.lock() {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime!");
            let _ = rt.block_on(client.execute(
                    "insert into logs (timestamp, level, target, message, module_path, file, line)
                                values ($1, $2, $3, $4, $5, $6, $7)",
                    &[
                        &(timestamp as i64),
                        &record.level().to_string(), 
                        &record.target().to_string(),
                        &record.args().to_string(),
                        &record.module_path().map(|s| s.to_string()),
                        &record.file().map(|s| s.to_string()),
                        &record.line()
                    ]
                )
            );
        }
    }

    fn flush(&self) {}
}
