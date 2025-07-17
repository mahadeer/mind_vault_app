use std::io;
use tracing::{subscriber, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, registry, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;

pub fn init_logger() -> WorkerGuard {
    // Create the logs directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all("./logs") {
        eprintln!("Warning: Could not create log directory './logs': {}", e);
    }
    // 1. Configure File Logging with .log extension at the end
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("mind_vault")
        .filename_suffix("json") // Removed leading dot, as it's a suffix, not an extension itself
        .build("./logs")
        .expect("Unable to create log appender");
    let (non_blocking_file_writer, guard) = tracing_appender::non_blocking(file_appender);

    // 2. Configure Console Logging Layer
    let stdout_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .compact();

    // 3. Configure File Logging Layer
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file_writer)
        .json()
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .flatten_event(true);

    // 4. Configure the EnvFilter
    let filter = EnvFilter::from_default_env()
        .add_directive(Level::INFO.into()) // Default global level
        .add_directive("mind_vault=debug".parse().expect("could not parse directive")); // Debug for your crate

    // 5. Build the global subscriber by combining filter and layers
    let log_subscriber = registry() // Start with a new registry
        .with(filter)           // Apply the filter globally
        .with(stdout_layer)     // Add the console output layer
        .with(file_layer);      // Add the file output layer

    // Set the constructed subscriber as the global default
    subscriber::set_global_default(log_subscriber)
        .expect("setting default subscriber failed");

    // Create the logs directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all("./logs") {
        eprintln!("Warning: Could not create log directory './logs': {}", e);
    }

    guard
}