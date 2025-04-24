use std::env::args;

enum LogLevel {
    Info,
    Debug,
    Error,
    Warning,
    Trace,
}

impl LogLevel {
    fn to_string(&self) -> &str {
        match self {
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Error => "Error",
            LogLevel::Warning => "Warning",
            LogLevel::Trace => "Trace",
        }
    }
}

// cargo run --bin first_enum -- debug
fn main() {
    let mut level = LogLevel::Info; // Default
    if let Some(level_arg) = args().nth(1) {
        level = match level_arg.as_str() {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "error" => LogLevel::Error,
            "warning" => LogLevel::Warning,
            "trace" => LogLevel::Trace,
            _ => {
                eprintln!("Invalid log level: {}", level_arg);
                std::process::exit(1);
            }
        };
    }

    println!("Log level: {}", &level.to_string());
}
