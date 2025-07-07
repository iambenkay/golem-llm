use std::{cell::RefCell, sync::Arc};

use client::Qdrant;
use conversions::get_api_key;
use golem_vector::{
    durability::DurableVector,
    vector::{
        analytics, collections, connection, namespaces, search, search_extended, types, vectors,
    },
};

mod client;
mod conversions;

struct QdrantComponent;

impl analytics::Guest for QdrantComponent {
    fn get_collection_stats(
        collection: String,
        namespace: Option<String>,
    ) -> Result<analytics::CollectionStats, types::VectorError> {
        todo!()
    }

    fn get_field_stats(
        collection: String,
        field: String,
        namespace: Option<String>,
    ) -> Result<analytics::FieldStats, types::VectorError> {
        todo!()
    }

    fn get_field_distribution(
        collection: String,
        field: String,
        limit: Option<u32>,
        namespace: Option<String>,
    ) -> Result<Vec<(types::MetadataValue, u64)>, types::VectorError> {
        todo!()
    }
}

impl collections::Guest for QdrantComponent {
    fn upsert_collection(
        name: String,
        description: Option<String>,
        dimension: u32,
        metric: types::DistanceMetric,
        index_config: Option<collections::IndexConfig>,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        todo!()
    }

    fn list_collections() -> Result<Vec<String>, types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.list_collections()
    }

    fn get_collection(name: String) -> Result<collections::CollectionInfo, types::VectorError> {
        todo!()
    }

    fn update_collection(
        name: String,
        description: Option<String>,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        todo!()
    }

    fn delete_collection(name: String) -> Result<(), types::VectorError> {
        todo!()
    }

    fn collection_exists(name: String) -> Result<bool, types::VectorError> {
        todo!()
    }
}

impl vectors::Guest for QdrantComponent {
    fn upsert_vectors(
        collection: String,
        vectors: Vec<types::VectorRecord>,
        namespace: Option<String>,
    ) -> Result<vectors::BatchResult, types::VectorError> {
        todo!()
    }

    fn upsert_vector(
        collection: String,
        id: types::Id,
        vector: types::VectorData,
        metadata: Option<types::Metadata>,
        namespace: Option<String>,
    ) -> Result<(), types::VectorError> {
        todo!()
    }

    fn get_vectors(
        collection: String,
        ids: Vec<types::Id>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::VectorRecord>, types::VectorError> {
        todo!()
    }

    fn get_vector(
        collection: String,
        id: types::Id,
        namespace: Option<String>,
    ) -> Result<Option<types::VectorRecord>, types::VectorError> {
        todo!()
    }

    fn update_vector(
        collection: String,
        id: types::Id,
        vector: Option<types::VectorData>,
        metadata: Option<types::Metadata>,
        namespace: Option<String>,
        merge_metadata: Option<bool>,
    ) -> Result<(), types::VectorError> {
        todo!()
    }

    fn delete_vectors(
        collection: String,
        ids: Vec<types::Id>,
        namespace: Option<String>,
    ) -> Result<u32, types::VectorError> {
        todo!()
    }

    fn delete_by_filter(
        collection: String,
        filter: types::FilterExpression,
        namespace: Option<String>,
    ) -> Result<u32, types::VectorError> {
        todo!()
    }

    fn delete_namespace(collection: String, namespace: String) -> Result<u32, types::VectorError> {
        todo!()
    }

    fn list_vectors(
        collection: String,
        namespace: Option<String>,
        filter: Option<types::FilterExpression>,
        limit: Option<u32>,
        cursor: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<vectors::ListResponse, types::VectorError> {
        todo!()
    }

    fn count_vectors(
        collection: String,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
    ) -> Result<u64, types::VectorError> {
        todo!()
    }
}

impl search::Guest for QdrantComponent {
    fn search_vectors(
        collection: String,
        query: search::SearchQuery,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        max_distance: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }

    fn find_similar(
        collection: String,
        vector: types::VectorData,
        limit: u32,
        namespace: Option<String>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }

    fn batch_search(
        collection: String,
        queries: Vec<search::SearchQuery>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<Vec<types::SearchResult>>, types::VectorError> {
        todo!()
    }
}

impl search_extended::Guest for QdrantComponent {
    fn recommend_vectors(
        collection: String,
        positive: Vec<search_extended::RecommendationExample>,
        negative: Option<Vec<search_extended::RecommendationExample>>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        strategy: Option<search_extended::RecommendationStrategy>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }

    fn discover_vectors(
        collection: String,
        context_pairs: Vec<search_extended::ContextPair>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }

    fn search_groups(
        collection: String,
        query: search::SearchQuery,
        group_by: String,
        group_size: u32,
        max_groups: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<search_extended::GroupedSearchResult>, types::VectorError> {
        todo!()
    }

    fn search_range(
        collection: String,
        vector: types::VectorData,
        min_distance: Option<f32>,
        max_distance: f32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        limit: Option<u32>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }

    fn search_text(
        collection: String,
        query_text: String,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        todo!()
    }
}

impl connection::Guest for QdrantComponent {
    fn connect(
        endpoint: String,
        credentials: Option<connection::Credentials>,
        timeout_ms: Option<u32>,
        _options: Option<types::Metadata>,
    ) -> Result<(), types::VectorError> {
        let api_key = get_api_key(credentials)?;

        let qdrant = Qdrant::new(endpoint, api_key, timeout_ms)?;

        qdrant.list_collections()?;
        set_qdrant_client(qdrant)?;
        Ok(())
    }

    fn disconnect() -> Result<(), types::VectorError> {
        unset_qdrant_client()
    }

    fn get_connection_status() -> Result<connection::ConnectionStatus, types::VectorError> {
        match get_qdrant_client() {
            Ok(qdrant) => {
                let connected = match qdrant.list_collections() {
                    Ok(..) => true,
                    Err(..) => false,
                };

                Ok(connection::ConnectionStatus {
                    connected,
                    connection_id: None,
                    endpoint: Some(qdrant.get_endpoint()),
                    last_activity: None,
                    provider: Some("Qdrant".to_string()),
                })
            }
            Err(..) => Ok(connection::ConnectionStatus {
                connected: false,
                connection_id: None,
                endpoint: None,
                last_activity: None,
                provider: Some("Qdrant".to_string()),
            }),
        }
    }

    fn test_connection(
        endpoint: String,
        credentials: Option<connection::Credentials>,
        timeout_ms: Option<u32>,
        _options: Option<types::Metadata>,
    ) -> Result<bool, types::VectorError> {
        let api_key = get_api_key(credentials)?;

        let qdrant = Qdrant::new(endpoint, api_key, timeout_ms)?;
        match qdrant.list_collections() {
            Ok(..) => Ok(true),
            Err(..) => Ok(false),
        }
    }
}

impl namespaces::Guest for QdrantComponent {
    fn upsert_namespace(
        collection: String,
        namespace: String,
        metadata: Option<types::Metadata>,
    ) -> Result<namespaces::NamespaceInfo, types::VectorError> {
        todo!()
    }

    fn list_namespaces(
        collection: String,
    ) -> Result<Vec<namespaces::NamespaceInfo>, types::VectorError> {
        todo!()
    }

    fn get_namespace(
        collection: String,
        namespace: String,
    ) -> Result<namespaces::NamespaceInfo, types::VectorError> {
        todo!()
    }

    fn delete_namespace(collection: String, namespace: String) -> Result<(), types::VectorError> {
        todo!()
    }

    fn namespace_exists(collection: String, namespace: String) -> Result<bool, types::VectorError> {
        todo!()
    }
}

fn get_qdrant_client() -> Result<Arc<Qdrant>, types::VectorError> {
    QDRANT_CLIENT.with_borrow(|client_opt| match client_opt {
        Some(client) => Ok(client.clone()),
        None => Err(types::VectorError::ConnectionError(
            "Qdrant client connection has not been configured".to_string(),
        )),
    })
}

fn set_qdrant_client(qdrant_client: Qdrant) -> Result<(), types::VectorError> {
    QDRANT_CLIENT.with_borrow_mut(|client_opt| match client_opt {
        Some(..) => Err(types::VectorError::AlreadyExists(
            "Connection has already been configured".to_string(),
        )),
        None => {
            let client = Arc::new(qdrant_client);
            *client_opt = Some(client.clone());
            Ok(())
        }
    })
}

fn unset_qdrant_client() -> Result<(), types::VectorError> {
    QDRANT_CLIENT.with_borrow_mut(|client_opt| match client_opt {
        Some(..) => {
            *client_opt = None;
            Ok(())
        }
        None => Err(types::VectorError::ProviderError(
            "There is no configured connection".to_string(),
        )),
    })
}

thread_local! {
    static QDRANT_CLIENT: RefCell<Option<Arc<Qdrant>>> = const { RefCell::new(None) };
}

type DurableQdrantComponent = DurableVector<QdrantComponent>;

golem_vector::export_vector!(DurableQdrantComponent with_types_in golem_vector);
