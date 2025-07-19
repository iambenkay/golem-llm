use std::str::FromStr;

pub struct BaseLoggingState {
    logging_initialized: bool,
}

impl BaseLoggingState {
    pub const fn new() -> Self {
        BaseLoggingState {
            logging_initialized: false,
        }
    }

    /// Initializes WASI logging based on the `GOLEM_LLM_LOG` environment variable.
    pub fn init(&mut self, var_name: &str) {
        if !self.logging_initialized {
            let _ = wasi_logger::Logger::install();
            let max_level: log::LevelFilter =
                log::LevelFilter::from_str(&std::env::var(var_name).unwrap_or_default())
                    .unwrap_or(log::LevelFilter::Info);
            log::set_max_level(max_level);
            self.logging_initialized = true;
        }
    }
}
