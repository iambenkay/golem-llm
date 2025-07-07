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

pub use crate::exports::golem;
pub use __export_llm_library_impl as export_llm;
