pub mod config;
pub mod durability;
pub mod error;

wit_bindgen::generate!({
    path: "../wit",
    world: "search-library",
    generate_all,
    generate_unused_types: true,
    additional_derives: [
        PartialEq,
        golem_rust::FromValueAndType,
        golem_rust::IntoValue
    ],
    pub_export_macro: true,
});

pub use crate::exports::golem;

pub use __export_search_library_impl as export_search;

use std::cell::RefCell;
use std::str::FromStr;

pub struct LoggingState {
    logging_initialized: bool,
}

impl LoggingState {
    pub fn init(&mut self) {
        if !self.logging_initialized {
            let _ = wasi_logger::Logger::install();
            let max_level: log::LevelFilter = log::LevelFilter::from_str(
                &std::env::var("SEARCH_PROVIDER_LOG_LEVEL").unwrap_or_default(),
            )
            .unwrap_or(log::LevelFilter::Info);
            log::set_max_level(max_level);
            self.logging_initialized = true;
        }
    }
}

thread_local! {
    /// This holds the state of our application.
    pub static LOGGING_STATE: RefCell<LoggingState> = const { RefCell::new(LoggingState {
        logging_initialized: false,
    }) };
}
