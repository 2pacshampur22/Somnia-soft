mod config;
use std::sync::Arc;
use tokio::time::{Sleep, Duration};
use log::{info, error, LevelFilter};
use env_logger::Builder;
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = config::AppConfig::new()
        .expect("failed to load config");
    let log_level = app_config.logging.level.parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);
    let mut log_builder = env_logger::Builder::new();
    log_builder.filter_level(log_level);
    if app_config.logging.enable_console_logging {
        log_builder.target(env_logger::Target::Stderr);
    }

    if app_config.logging.enable_file_logging {
        if let Some(log_file_path_str) = &app_config.logging.log_file_path {
            let log_file_path = std::path::PathBuf::from(log_file_path_str);
            let log_dir = log_file_path.parent().unwrap_or_else(|| std::path::Path::new("."));
            tokio::fs::create_dir_all(log_dir).await?;
            let file = std::fs::File::create(&log_file_path)
                .map_err(|e| anyhow!("Failed to create log file at {}: {}", log_file_path.display(), e))?;
            log_builder.target(env_logger::Target::Pipe(Box::new(file)));
        }
        else {
            error!("File logging is enabled but path is not specified in config");
        }
    }
    log_builder.init();
        
    Ok(())
}