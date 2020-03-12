use slog::{
    error,
    info,
};

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
}

pub fn run(working_directory: &str, logger: &slog::Logger) -> i32 {
    let directory = std::path::Path::new(working_directory);
    if directory.exists() {
        if !directory.is_dir() {
            error!(logger, "Working directory path \"{}\" does not actually target a directory", working_directory);
            return 1;
        }
    } else {
        info!(logger, "Creating working directory at \"{}\"", working_directory);
        if !std::fs::create_dir(&working_directory).is_ok() {
            error!(logger, "Unable to create directory \"{}\"", working_directory);
            return 1;
        }
    }
    return 0;
}
