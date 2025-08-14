use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::{DateTime, Local};
use log::{LevelFilter, Metadata, Record};

pub struct FileLogger {
    file: Mutex<File>,
    level: LevelFilter,
}

impl FileLogger {
    pub fn new(log_path: &str, level: LevelFilter) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Failed to open log file");

        Self {
            file: Mutex::new(file),
            level,
        }
    }

    pub fn init(log_path: &str, level: LevelFilter) {
        let logger = Box::new(FileLogger::new(log_path, level));
        log::set_boxed_logger(logger).expect("Failed to set logger");
        log::set_max_level(level);
    }
}

impl log::Log for FileLogger {
    fn enabled(&self,
        metadata: &Metadata,
    ) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self,
        record: &Record,
    ) {
        if self.enabled(record.metadata()) {
            let now: DateTime<Local> = Local::now();
            let log_entry = format!(
                "[{}] {} - {}:{} - {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            );

            if let Ok(mut file) = self.file.lock() {
                let _ = writeln!(file, "{}", log_entry);
            }
        }
    }

    fn flush(&self,
    ) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

pub fn get_log_dir() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("logs");
    
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create log directory");
    }
    
    path
}

pub fn get_log_file() -> String {
    let mut log_dir = get_log_dir();
    log_dir.push(format!("windows-env-manager-{}.log", Local::now().format("%Y-%m-%d")));
    log_dir.to_string_lossy().to_string()
}

pub fn init_logger() {
    let log_file = get_log_file();
    FileLogger::init(&log_file, LevelFilter::Info);
    log::info!("Logger initialized at: {}", log_file);
}