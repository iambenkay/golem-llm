use golem_vector::{
    error::from_reqwest_error,
    vector::{collections, types},
};

use crate::conversions::get_response_body;

pub struct Qdrant {
    endpoint: String,
    client: reqwest::Client,
}

impl Qdrant {
    pub fn new(
        endpoint: String,
        api_key: Option<String>,
        timeout_ms: Option<u32>,
    ) -> Result<Self, types::VectorError> {
        let timeout = timeout_ms
            .or(Some(30000))
            .map(|ms| std::time::Duration::from_millis(ms as u64));

        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(api_key) = api_key {
            headers.insert(
                reqwest::header::HeaderName::from_static("api-key"),
                reqwest::header::HeaderValue::from_str(&api_key).map_err(|e| {
                    types::VectorError::Unauthorized(format!(
                        "The API key you supplied is not valid"
                    ))
                })?,
            );
        }

        Ok(Self {
            endpoint,
            client: reqwest::Client::builder()
                .timeout(timeout)
                .default_headers(headers)
                .build()
                .map_err(|e| types::VectorError::ConnectionError(e.to_string()))?,
        })
    }

    pub fn get_endpoint(&self) -> String {
        self.endpoint.to_owned()
    }

    pub fn list_collections(&self) -> Result<Vec<String>, types::VectorError> {
        let response = self
            .client
            .get(format!("{}/collections", self.endpoint))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<Vec<CollectionName>> = get_response_body(response)?;
        Ok(response.result.into_iter().map(|c| c.name).collect())
    }

    pub fn get_collection(
        &self,
        name: String,
    ) -> Result<Vec<collections::CollectionInfo>, types::VectorError> {
        let response = self
            .client
            .get(format!("{}/collections/{name}", self.endpoint))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<Vec<CollectionName>> = get_response_body(response)?;

        Ok(response.result.into_iter().map(|c| c.name).collect())
    }
}

#[derive(serde::Deserialize)]
struct QdrantResponse<T> {
    usage: QdrantUsage,
    result: T,
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "time")]
    _time: f64,
}

#[derive(serde::Deserialize)]
struct QdrantUsage {
    cpu: u64,
    payload_io_read: u64,
    payload_io_write: u64,
    payload_index_io_read: u64,
    payload_index_io_write: u64,
    vector_io_read: u64,
    vector_io_write: u64,
}

#[derive(serde::Deserialize)]
struct CollectionName {
    name: String,
}

#[derive(serde::Deserialize)]
struct CollectionInformation {
    status: String,
    optimizer_status: String,
    segments_count: u64,
    points_count: u64,
    vectors_count: u64,
    indexed_vectors_count: u64,
}
