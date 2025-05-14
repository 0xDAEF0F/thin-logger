# env-logger-wrapper

A wrapper around Rust's `env_logger` with better defaults and colorized output.

## Features

- Colorized log levels and timestamps
- Simplified module path format
- Sensible default configuration
- Re-exports from `env_logger`, `log`, and `colored` for convenience

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
env-logger-wrapper = "x.x.x"
```

or run `cargo add env-logger-wrapper`

Basic usage:

```rust
use env_logger_wrapper::{new_builder, LevelFilter};

fn main() {
    // Initialize the logger with trace level
    new_builder(LevelFilter::Trace).init();

    // Use standard log macros
    log::trace!("This is a trace message");
    log::debug!("This is a debug message");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
}
```

## Example Output

The logger outputs in the format:

```
[HH:MM:SSam/pm] [LEVEL] [module_name]: message
```

With colorized timestamps and log levels for improved readability.

## License

MIT
