use std::path::Path;
use std::time::{Duration, SystemTime};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

use super::paths::log_dir;

const LOG_RETENTION_DAYS: u64 = 7;

pub fn init_logging() {
    let dir = log_dir();
    cleanup_old_logs(&dir);

    let file_appender = rolling::daily(&dir, "mimi-ime.log");
    fmt()
        .with_writer(file_appender)
        .with_env_filter(EnvFilter::new("debug"))
        .with_target(true)
        .with_line_number(true)
        .init();
}

fn cleanup_old_logs(dir: &Path) {
    let cutoff = match SystemTime::now()
        .checked_sub(Duration::from_secs(LOG_RETENTION_DAYS * 24 * 60 * 60))
    {
        Some(t) => t,
        None => return,
    };

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("log cleanup: cannot read {}: {}", dir.display(), e);
            return;
        }
    };

    let mut removed = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if !name.starts_with("mimi-ime.log") {
            continue;
        }
        let modified = match entry.metadata().and_then(|m| m.modified()) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if modified < cutoff && std::fs::remove_file(&path).is_ok() {
            removed += 1;
        }
    }

    if removed > 0 {
        eprintln!(
            "log cleanup: removed {} file(s) older than {} days",
            removed, LOG_RETENTION_DAYS
        );
    }
}
