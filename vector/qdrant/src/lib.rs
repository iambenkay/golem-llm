use std::{cell::RefCell, sync::Arc};

use client::Qdrant;
use conversions::get_api_key;
use golem_utils::{
    durability::{read_remote_durably, write_remote_durably},
    params::{Param, Param2, Param5},
};
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
        _collection: String,
        _namespace: Option<String>,
    ) -> Result<analytics::CollectionStats, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "analytics::get_collection_stats is not supported by Qdrant".to_string(),
        ))
    }

    fn get_field_stats(
        _collection: String,
        _field: String,
        _namespace: Option<String>,
    ) -> Result<analytics::FieldStats, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "analytics::get_field_stats is not supported by Qdrant".to_string(),
        ))
    }

    fn get_field_distribution(
        _collection: String,
        _field: String,
        _limit: Option<u32>,
        _namespace: Option<String>,
    ) -> Result<Vec<(types::MetadataValue, u64)>, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "analytics::get_field_distribution is not supported by Qdrant".to_string(),
        ))
    }
}

impl collections::Guest for QdrantComponent {
    fn upsert_collection(
        name: String,
        _description: Option<String>,
        dimension: u32,
        metric: types::DistanceMetric,
        index_config: Option<collections::IndexConfig>,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        let exists = read_remote_durably(
            "golem_vector::collections",
            "upsert_collection/collection_exists",
            Param::new(name.clone()),
            |params| qdrant.collection_exists(params.p1),
        )?;

        if exists {
            write_remote_durably(
                "golem_vector::collections",
                "upsert_collection/update_collection",
                Param2::new(name, metadata),
                |params| qdrant.update_collection(params.p1, params.p2),
            )
        } else {
            write_remote_durably(
                "golem_vector::collections",
                "upsert_collection/create_collection",
                Param5::new(name, dimension, metric, index_config, metadata),
                |params| {
                    qdrant.create_collection(params.p1, params.p2, params.p3, params.p4, params.p5)
                },
            )
        }
    }

    fn list_collections() -> Result<Vec<String>, types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.list_collections()
    }

    fn get_collection(name: String) -> Result<collections::CollectionInfo, types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.get_collection(name)
    }

    fn update_collection(
        name: String,
        _description: Option<String>,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.update_collection(name, metadata)
    }

    fn delete_collection(name: String) -> Result<(), types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.delete_collection(name)
    }

    fn collection_exists(name: String) -> Result<bool, types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.collection_exists(name)
    }
}

impl vectors::Guest for QdrantComponent {
    fn upsert_vectors(
        collection: String,
        vectors: Vec<types::VectorRecord>,
        _namespace: Option<String>,
    ) -> Result<vectors::BatchResult, types::VectorError> {
        let qdrant = get_qdrant_client()?;
        qdrant.upsert_points(collection, vectors)
    }

    fn upsert_vector(
        collection: String,
        id: types::Id,
        vector: types::VectorData,
        metadata: Option<types::Metadata>,
        _namespace: Option<String>,
    ) -> Result<(), types::VectorError> {
        let qdrant = get_qdrant_client()?;

        let vectors = vec![types::VectorRecord {
            id,
            vector,
            metadata,
        }];
        qdrant.upsert_points(collection, vectors)?;

        Ok(())
    }

    fn get_vectors(
        collection: String,
        ids: Vec<types::Id>,
        _namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::VectorRecord>, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.retrieve_points(collection, ids, include_vectors, include_metadata)
    }

    fn get_vector(
        collection: String,
        id: types::Id,
        _namespace: Option<String>,
    ) -> Result<Option<types::VectorRecord>, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.retrieve_point(collection, id)
    }

    fn update_vector(
        collection: String,
        id: types::Id,
        vector: Option<types::VectorData>,
        metadata: Option<types::Metadata>,
        _namespace: Option<String>,
        merge_metadata: Option<bool>,
    ) -> Result<(), types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.update_vectors(collection, id, vector, metadata, merge_metadata)
    }

    fn delete_vectors(
        collection: String,
        ids: Vec<types::Id>,
        _namespace: Option<String>,
    ) -> Result<u32, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.delete_points(collection, Some(ids), None)
    }

    fn delete_by_filter(
        collection: String,
        filter: types::FilterExpression,
        _namespace: Option<String>,
    ) -> Result<u32, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.delete_points(collection, None, Some(filter))
    }

    fn delete_namespace(
        _collection: String,
        _namespace: String,
    ) -> Result<u32, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "vectors::delete_namespace is not supported by Qdrant".to_string(),
        ))
    }

    fn list_vectors(
        collection: String,
        _namespace: Option<String>,
        filter: Option<types::FilterExpression>,
        limit: Option<u32>,
        cursor: Option<types::Id>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<vectors::ListResponse, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.scroll_points(
            collection,
            filter,
            limit,
            cursor,
            include_vectors,
            include_metadata,
        )
    }

    fn count_vectors(
        collection: String,
        filter: Option<types::FilterExpression>,
        _namespace: Option<String>,
    ) -> Result<u64, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.count_points(collection, filter)
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
        _max_distance: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.search_points(
            collection,
            query,
            limit,
            filter,
            namespace,
            include_vectors,
            include_metadata,
            min_score,
            search_params,
        )
    }

    fn find_similar(
        collection: String,
        vector: types::VectorData,
        limit: u32,
        namespace: Option<String>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let qdrant = get_qdrant_client()?;

        qdrant.search_points(
            collection,
            search::SearchQuery::Vector(vector),
            limit,
            None,
            namespace,
            Some(true),
            Some(true),
            None,
            None,
        )
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
        let qdrant = get_qdrant_client()?;

        qdrant.batch_query_points(
            collection,
            queries,
            limit,
            filter,
            namespace,
            include_vectors,
            include_metadata,
            None,
            search_params,
        )
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
        let qdrant = get_qdrant_client()?;

        qdrant.recommend_points(
            collection,
            positive,
            negative,
            limit,
            filter,
            namespace,
            include_vectors,
            include_metadata,
            strategy,
        )
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
        let qdrant = get_qdrant_client()?;

        qdrant.discover_points(
            collection,
            context_pairs,
            limit,
            filter,
            namespace,
            include_vectors,
            include_metadata,
        )
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
        let qdrant = get_qdrant_client()?;

        qdrant.query_point_groups(
            collection,
            query,
            group_by,
            group_size,
            max_groups,
            filter,
            namespace,
            include_vectors,
            include_metadata,
        )
    }

    fn search_range(
        _collection: String,
        _vector: types::VectorData,
        _min_distance: Option<f32>,
        _max_distance: f32,
        _filter: Option<types::FilterExpression>,
        _namespace: Option<String>,
        _limit: Option<u32>,
        _include_vectors: Option<bool>,
        _include_metadata: Option<bool>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "search_extended::search_range is not supported by Qdrant".to_string(),
        ))
    }

    fn search_text(
        _collection: String,
        _query_text: String,
        _limit: u32,
        _filter: Option<types::FilterExpression>,
        _namespace: Option<String>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "search_extended::search_text is not supported by Qdrant".to_string(),
        ))
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
        _collection: String,
        _namespace: String,
        _metadata: Option<types::Metadata>,
    ) -> Result<namespaces::NamespaceInfo, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "namespaces::upsert_namespace is not supported by Qdrant".to_string(),
        ))
    }

    fn list_namespaces(
        _collection: String,
    ) -> Result<Vec<namespaces::NamespaceInfo>, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "namespaces::list_namespaces is not supported by Qdrant".to_string(),
        ))
    }

    fn get_namespace(
        _collection: String,
        _namespace: String,
    ) -> Result<namespaces::NamespaceInfo, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "namespaces::get_namespace is not supported by Qdrant".to_string(),
        ))
    }

    fn delete_namespace(_collection: String, _namespace: String) -> Result<(), types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "namespaces::delete_namespace is not supported by Qdrant".to_string(),
        ))
    }

    fn namespace_exists(
        _collection: String,
        _namespace: String,
    ) -> Result<bool, types::VectorError> {
        Err(types::VectorError::UnsupportedFeature(
            "namespaces::namespace_exists is not supported by Qdrant".to_string(),
        ))
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
