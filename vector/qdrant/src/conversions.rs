use golem_vector::{
    error::from_reqwest_error,
    vector::{collections, connection, types},
};
use qdrant_client::{qdrant::Distance, QdrantError};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub fn get_api_key(
    credentials: Option<connection::Credentials>,
) -> Result<Option<String>, types::VectorError> {
    match credentials {
        Some(connection::Credentials::ApiKey(api_key)) => Ok(Some(api_key)),
        None => Ok(None),
        Some(..) => Err(types::VectorError::InvalidParams(
            "Invalid credential type supplied. Requires `ApiKey`".to_string(),
        )),
    }
}

pub fn from_qdrant_error(value: QdrantError) -> types::VectorError {
    match value {
            QdrantError::ResponseError { status } => types::VectorError::ProviderError(format!(
                "An error was returned by Qdrant - code: {}, message: {}, metadata: {:?}",
                status.code(),
                status.message(),
                status.metadata()
            )),
            QdrantError::ResourceExhaustedError {
                status,
                retry_after_seconds,
            } => types::VectorError::ProviderError(format!("A resource exhausted error was returned by Qdrant - code: {}, message: {}, metadata: {:?}, retry after {retry_after_seconds} seconds", 
            status.code(),
            status.message(),
            status.metadata())),
            QdrantError::ConversionError(error) => types::VectorError::ProviderError(format!("A conversion error occurred: {error}")),
            QdrantError::InvalidUri(invalid_uri) => types::VectorError::InvalidParams(format!("An invalid Qdrant server URL was provided: {invalid_uri}")),
            QdrantError::NoSnapshotFound(snapshot) => types::VectorError::NotFound(format!("The requested snapshot was not found: {snapshot}")),
            QdrantError::Io(error) => types::VectorError::ProviderError(format!("An IO Error occurred: {error}")),
            QdrantError::Reqwest(error) => types::VectorError::ProviderError(format!("An API request error occurred: {error}")),
            QdrantError::JsonToPayload(value) => types::VectorError::ProviderError(format!("An unsupported json value was received instead of an object: {value}")),
        }
}

pub fn json_to_metadata(json: serde_json::Value) -> types::MetadataValue {
    match json {
        serde_json::Value::Null => types::MetadataValue::NullVal,
        serde_json::Value::Bool(boolean) => types::MetadataValue::BooleanVal(boolean),
        serde_json::Value::Number(number) => {
            if number.is_f64() {
                types::MetadataValue::NumberVal(number.as_f64().unwrap())
            } else {
                types::MetadataValue::IntegerVal(
                    number.as_i64().unwrap_or(number.as_u64().unwrap() as i64),
                )
            }
        }
        serde_json::Value::String(string) => types::MetadataValue::StringVal(string),
        serde_json::Value::Array(arr) => types::MetadataValue::ArrayVal(
            arr.into_iter()
                .map(|v| types::LazyMetadataValue::new(json_to_metadata(v)))
                .collect(),
        ),
        serde_json::Value::Object(map) => types::MetadataValue::ObjectVal(
            map.into_iter()
                .map(|(k, v)| (k, types::LazyMetadataValue::new(json_to_metadata(v))))
                .collect(),
        ),
    }
}

pub fn from_qdrant_collection_info(name: String, info: qdrant_client::qdrant::CollectionInfo) -> collections::CollectionInfo {
    Distance
    collections::CollectionInfo {
        name,
        description: None,
        dimension: info.,
        metric: info,
        vector_count: info.vectors_count(),
        size_bytes: None,
        index_ready: info.indexed_vectors_count() > 0,
        created_at: None,
        updated_at: None,
        provider_stats: ,
    }
}