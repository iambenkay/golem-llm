use golem_vector::vector::{collections, types};
use qdrant_client::Qdrant;
use wit_bindgen_rt::async_support::futures::TryFutureExt;

use crate::conversions::from_qdrant_error;

pub struct QdrantService {
    client: Qdrant,
}

impl QdrantService {
    pub fn new(
        endpoint: String,
        api_key: Option<String>,
        timeout_ms: Option<u32>,
    ) -> Result<Self, types::VectorError> {
        let timeout = timeout_ms.unwrap_or(30000) as u64;

        let mut qdrant_client = Qdrant::from_url(&endpoint);

        if let Some(api_key) = api_key {
            qdrant_client = qdrant_client.api_key(api_key)
        }

        let client = qdrant_client
            .connect_timeout(timeout)
            .build()
            .map_err(from_qdrant_error)?;

        Ok(Self { client })
    }

    pub fn get_endpoint(&self) -> String {
        self.client.config.uri.to_owned()
    }

    pub async fn list_collections(&self) -> Result<Vec<String>, types::VectorError> {
        let response = self
            .client
            .list_collections()
            .await
            .map_err(from_qdrant_error)?;

        Ok(response
            .collections
            .into_iter()
            .map(|desc| desc.name)
            .collect::<Vec<String>>())
    }

    pub async fn get_collection(
        &self,
        name: String,
    ) -> Result<Vec<collections::CollectionInfo>, types::VectorError> {
        let response = self
            .client
            .collection_info(name)
            .await
            .map_err(from_qdrant_error)?;

        self.client.co

        match response.result {
            None => Err(types::VectorError::NotFound(format!(
                "The collection you requested was not found: {name}"
            ))),
            Some(info) => {}
        }

        Ok()
    }
}
