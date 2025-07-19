pub mod chat_stream;
pub mod config;
pub mod durability;
pub mod error;

#[allow(dead_code)]
pub mod event_source;

wit_bindgen::generate!({
    path: "../wit",
    world: "llm-library",
    generate_all,
    generate_unused_types: true,
    additional_derives: [PartialEq, golem_rust::FromValueAndType, golem_rust::IntoValue],
    pub_export_macro: true,
});

pub struct LoggingState {
    base_logger: BaseLoggingState,
}

impl LoggingState {
    /// Initializes WASI logging based on the `GOLEM_LLM_LOG` environment variable.
    pub fn init(&mut self) {
        self.base_logger.init("GOLEM_LLM_LOG");
    }
}

thread_local! {
    /// This holds the state of our application.
    pub static LOGGING_STATE: RefCell<LoggingState> = const { RefCell::new(LoggingState {
        base_logger: BaseLoggingState::new(),
    }) };
}

use std::cell::RefCell;

pub use crate::exports::golem;
pub use __export_llm_library_impl as export_llm;
use golem_utils::BaseLoggingState;
