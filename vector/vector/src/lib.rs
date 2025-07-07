use exports::golem::vector::types::{self, GuestLazyFilterExpression, GuestLazyMetadataValue};
use golem_rust::value_and_type::{FromValueAndType, IntoValue, TypeNodeBuilder};
use golem_rust::wasm_rpc::{NodeBuilder, ResourceMode, Uri};

pub mod durability;
pub mod error;

const LAZY_METADATA_VALUE: u64 = 0x000001;
const LAZY_FILTER_EXPRESSION: u64 = 0x000002;

wit_bindgen::generate!({
    path: "../wit",
    world: "vector-library",
    generate_all,
    generate_unused_types: true,
    additional_derives: [PartialEq, Clone, golem_rust::FromValueAndType, golem_rust::IntoValue],
    pub_export_macro: true,
});

impl Clone for types::LazyFilterExpression {
    fn clone(&self) -> Self {
        Self::new(self.get::<types::FilterExpression>().clone())
    }
}

impl PartialEq for types::LazyFilterExpression {
    fn eq(&self, other: &Self) -> bool {
        self.get::<types::FilterExpression>() == other.get::<types::FilterExpression>()
    }
}

impl FromValueAndType for types::LazyFilterExpression {
    fn from_extractor<'a, 'b>(
        extractor: &'a impl golem_rust::wasm_rpc::WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        types::FilterExpression::from_extractor(extractor).map(Self::new)
    }
}

impl IntoValue for types::LazyFilterExpression {
    fn add_to_builder<B: NodeBuilder>(self, builder: B) -> B::Result {
        builder.handle(
            Uri {
                value: "golem::vector::lazy-filter-expression".to_string(),
            },
            self.handle() as u64,
        )
    }

    fn add_to_type_builder<B: TypeNodeBuilder>(builder: B) -> B::Result {
        builder.handle(LAZY_FILTER_EXPRESSION, ResourceMode::Owned)
    }
}

impl GuestLazyFilterExpression for types::FilterExpression {
    fn get(&self) -> types::FilterExpression {
        self.clone()
    }
}

impl Clone for types::LazyMetadataValue {
    fn clone(&self) -> Self {
        Self::new(self.get::<types::MetadataValue>().clone())
    }
}

impl PartialEq for types::LazyMetadataValue {
    fn eq(&self, other: &Self) -> bool {
        self.get::<types::MetadataValue>() == other.get::<types::MetadataValue>()
    }
}

impl FromValueAndType for types::LazyMetadataValue {
    fn from_extractor<'a, 'b>(
        extractor: &'a impl golem_rust::wasm_rpc::WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        types::MetadataValue::from_extractor(extractor).map(Self::new)
    }
}

impl IntoValue for types::LazyMetadataValue {
    fn add_to_builder<B: NodeBuilder>(self, builder: B) -> B::Result {
        builder.handle(
            Uri {
                value: "golem::vector::lazy-metadata-value".to_string(),
            },
            self.handle() as u64,
        )
    }

    fn add_to_type_builder<B: TypeNodeBuilder>(builder: B) -> B::Result {
        builder.handle(LAZY_METADATA_VALUE, ResourceMode::Owned)
    }
}

impl GuestLazyMetadataValue for types::MetadataValue {
    fn get(&self) -> types::MetadataValue {
        self.clone()
    }
}

pub use crate::exports::golem::vector;
pub use __export_vector_library_impl as export_vector;
