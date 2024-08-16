// Re-exports for handy usage and shared version usage.
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

fn is_running_on_cloud_run() -> bool {
    std::env::var("K_SERVICE").is_ok() || std::env::var("CLOUD_RUN_JOB").is_ok()
}

/// Init defaults for the service calling this.
pub fn init() {
    let base_filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    if is_running_on_cloud_run() {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(base_filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(base_filter)
            .init();
    }
}
