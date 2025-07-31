use crate::vector::analytics::{CollectionStats, FieldStats, Guest as AnalyticsGuest};
use crate::vector::collections::{CollectionInfo, Guest as CollectionsGuest, IndexConfig};
use crate::vector::connection::{ConnectionStatus, Credentials, Guest as ConnectionGuest};
use crate::vector::namespaces::{Guest as NamespacesGuest, NamespaceInfo};
use crate::vector::search::{Guest as SearchGuest, SearchQuery};
use crate::vector::search_extended::{
    ContextPair, GroupedSearchResult, Guest as SearchExtendedGuest, RecommendationExample,
    RecommendationStrategy,
};
use crate::vector::types::{
    DistanceMetric, FilterExpression, Guest as TypesGuest, Id, Metadata, MetadataValue,
    SearchResult, VectorData, VectorError, VectorRecord,
};
use crate::vector::vectors::{BatchResult, Guest as VectorsGuest, ListResponse};
use golem_utils::durability::write_remote_durably_with_side_effects;
use golem_utils::{
    durability::{read_remote_durably, write_remote_durably},
    params::{
        NoParam, Param, Param10, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9,
    },
};
use std::marker::PhantomData;

pub struct DurableVector<Impl> {
    phantom: PhantomData<Impl>,
}

impl<Impl> TypesGuest for DurableVector<Impl> {
    type LazyMetadataValue = MetadataValue;
    type LazyFilterExpression = FilterExpression;
}

impl<Impl: ConnectionGuest> ConnectionGuest for DurableVector<Impl> {
    #[doc = "Establish connection to vector database"]
    fn connect(
        endpoint: String,
        credentials: Option<Credentials>,
        timeout_ms: Option<u32>,
        options: Option<Metadata>,
    ) -> Result<(), VectorError> {
        read_remote_durably(
            "golem_vector::connection",
            "connect",
            Param4::new(endpoint, credentials, timeout_ms, options),
            |params| params.invoke(Impl::connect),
        )
    }

    #[doc = "Close connection"]
    fn disconnect() -> Result<(), VectorError> {
        read_remote_durably(
            "golem_vector::connection",
            "disconnect",
            NoParam,
            |params| params.invoke(Impl::disconnect),
        )
    }

    #[doc = "Get current connection status"]
    fn get_connection_status() -> Result<ConnectionStatus, VectorError> {
        read_remote_durably(
            "golem_vector::connection",
            "get_connection_status",
            NoParam,
            |params| params.invoke(Impl::get_connection_status),
        )
    }

    #[doc = "Test connection without modifying state"]
    fn test_connection(
        endpoint: String,
        credentials: Option<Credentials>,
        timeout_ms: Option<u32>,
        options: Option<Metadata>,
    ) -> Result<bool, VectorError> {
        read_remote_durably(
            "golem_vector::connection",
            "test_connection",
            Param4::new(endpoint, credentials, timeout_ms, options),
            |params| params.invoke(Impl::test_connection),
        )
    }
}

impl<Impl: NamespacesGuest> NamespacesGuest for DurableVector<Impl> {
    #[doc = "Create or update namespace (upsert)"]
    fn upsert_namespace(
        collection: String,
        namespace: String,
        metadata: Option<Metadata>,
    ) -> Result<NamespaceInfo, VectorError> {
        write_remote_durably(
            "golem_vector::namespaces",
            "upsert_namespace",
            Param3::new(collection, namespace, metadata),
            |params| params.invoke(Impl::upsert_namespace),
        )
    }

    #[doc = "List namespaces in collection"]
    fn list_namespaces(collection: String) -> Result<Vec<NamespaceInfo>, VectorError> {
        read_remote_durably(
            "golem_vector::namespaces",
            "list_namespaces",
            Param::new(collection),
            |params| params.invoke(Impl::list_namespaces),
        )
    }

    #[doc = "Get namespace information"]
    fn get_namespace(collection: String, namespace: String) -> Result<NamespaceInfo, VectorError> {
        read_remote_durably(
            "golem_vector::namespaces",
            "get_namespace",
            Param2::new(collection, namespace),
            |params| params.invoke(Impl::get_namespace),
        )
    }

    #[doc = "Delete namespace and all vectors within it"]
    fn delete_namespace(collection: String, namespace: String) -> Result<(), VectorError> {
        write_remote_durably(
            "golem_vector::namespaces",
            "delete_namespace",
            Param2::new(collection, namespace),
            |params| params.invoke(Impl::delete_namespace),
        )
    }

    #[doc = "Check if namespace exists"]
    fn namespace_exists(collection: String, namespace: String) -> Result<bool, VectorError> {
        read_remote_durably(
            "golem_vector::namespaces",
            "namespace_exists",
            Param2::new(collection, namespace),
            |params| params.invoke(Impl::namespace_exists),
        )
    }
}

impl<Impl: AnalyticsGuest> AnalyticsGuest for DurableVector<Impl> {
    #[doc = "Get collection statistics"]
    fn get_collection_stats(
        collection: String,
        namespace: Option<String>,
    ) -> Result<CollectionStats, VectorError> {
        read_remote_durably(
            "golem_vector::analytics",
            "get_collection_stats",
            Param2::new(collection, namespace),
            |params| params.invoke(Impl::get_collection_stats),
        )
    }

    #[doc = "Get field statistics"]
    fn get_field_stats(
        collection: String,
        field: String,
        namespace: Option<String>,
    ) -> Result<FieldStats, VectorError> {
        read_remote_durably(
            "golem_vector::analytics",
            "get_field_stats",
            Param3::new(collection, field, namespace),
            |params| params.invoke(Impl::get_field_stats),
        )
    }

    #[doc = "Get value distribution for a field"]
    fn get_field_distribution(
        collection: String,
        field: String,
        limit: Option<u32>,
        namespace: Option<String>,
    ) -> Result<Vec<(MetadataValue, u64)>, VectorError> {
        read_remote_durably(
            "golem_vector::analytics",
            "get_field_distribution",
            Param4::new(collection, field, limit, namespace),
            |params| params.invoke(Impl::get_field_distribution),
        )
    }
}

impl<Impl: SearchExtendedGuest> SearchExtendedGuest for DurableVector<Impl> {
    #[doc = "Recommendation-based search"]
    fn recommend_vectors(
        collection: String,
        positive: Vec<RecommendationExample>,
        negative: Option<Vec<RecommendationExample>>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        strategy: Option<RecommendationStrategy>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search-extended",
            "recommend_vectors",
            Param9::new(
                collection,
                positive,
                negative,
                limit,
                filter,
                namespace,
                strategy,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::recommend_vectors),
        )
    }

    #[doc = "Discovery/context-based search"]
    fn discover_vectors(
        collection: String,
        context_pairs: Vec<ContextPair>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search-extended",
            "discover_vectors",
            Param7::new(
                collection,
                context_pairs,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::discover_vectors),
        )
    }

    #[doc = "Grouped search for diverse results"]
    fn search_groups(
        collection: String,
        query: SearchQuery,
        group_by: String,
        group_size: u32,
        max_groups: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<GroupedSearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search-extended",
            "search_groups",
            Param9::new(
                collection,
                query,
                group_by,
                group_size,
                max_groups,
                filter,
                namespace,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::search_groups),
        )
    }

    #[doc = "Range search within distance bounds"]
    fn search_range(
        collection: String,
        vector: VectorData,
        min_distance: Option<f32>,
        max_distance: f32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        limit: Option<u32>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search-extended",
            "search_range",
            Param9::new(
                collection,
                vector,
                min_distance,
                max_distance,
                filter,
                namespace,
                limit,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::search_range),
        )
    }

    #[doc = "Text/document search (auto-embedding)"]
    fn search_text(
        collection: String,
        query_text: String,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search-extended",
            "search_text",
            Param5::new(collection, query_text, limit, filter, namespace),
            |params| params.invoke(Impl::search_text),
        )
    }
}

impl<Impl: SearchGuest> SearchGuest for DurableVector<Impl> {
    #[doc = "Similarity search"]
    fn search_vectors(
        collection: String,
        query: SearchQuery,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        max_distance: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search",
            "search_vectors",
            Param10::new(
                collection,
                query,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
                min_score,
                max_distance,
                search_params,
            ),
            |params| params.invoke(Impl::search_vectors),
        )
    }

    #[doc = "Simple vector similarity search (convenience)"]
    fn find_similar(
        collection: String,
        vector: VectorData,
        limit: u32,
        namespace: Option<String>,
    ) -> Result<Vec<SearchResult>, VectorError> {
        read_remote_durably(
            "golem_vector::search",
            "find_similar",
            Param4::new(collection, vector, limit, namespace),
            |params| params.invoke(Impl::find_similar),
        )
    }

    #[doc = "Batch similarity search"]
    fn batch_search(
        collection: String,
        queries: Vec<SearchQuery>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<Vec<SearchResult>>, VectorError> {
        read_remote_durably(
            "golem_vector::search",
            "batch_search",
            Param8::new(
                collection,
                queries,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
                search_params,
            ),
            |params| params.invoke(Impl::batch_search),
        )
    }
}

impl<Impl: VectorsGuest> VectorsGuest for DurableVector<Impl> {
    #[doc = "Upsert vectors into collection"]
    fn upsert_vectors(
        collection: String,
        vectors: Vec<VectorRecord>,
        namespace: Option<String>,
    ) -> Result<BatchResult, VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "upsert_vectors",
            Param3::new(collection, vectors, namespace),
            |params| params.invoke(Impl::upsert_vectors),
        )
    }

    #[doc = "Upsert single vector (convenience)"]
    fn upsert_vector(
        collection: String,
        id: Id,
        vector: VectorData,
        metadata: Option<Metadata>,
        namespace: Option<String>,
    ) -> Result<(), VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "upsert_vector",
            Param5::new(collection, id, vector, metadata, namespace),
            |params| params.invoke(Impl::upsert_vector),
        )
    }

    #[doc = "Get vectors by IDs"]
    fn get_vectors(
        collection: String,
        ids: Vec<Id>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<VectorRecord>, VectorError> {
        read_remote_durably(
            "golem_vector::vectors",
            "get_vectors",
            Param5::new(
                collection,
                ids,
                namespace,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::get_vectors),
        )
    }

    #[doc = "Get single vector by ID (convenience)"]
    fn get_vector(
        collection: String,
        id: Id,
        namespace: Option<String>,
    ) -> Result<Option<VectorRecord>, VectorError> {
        read_remote_durably(
            "golem_vector::vectors",
            "get_vector",
            Param3::new(collection, id, namespace),
            |params| params.invoke(Impl::get_vector),
        )
    }

    #[doc = "Update vector in place"]
    fn update_vector(
        collection: String,
        id: Id,
        vector: Option<VectorData>,
        metadata: Option<Metadata>,
        namespace: Option<String>,
        merge_metadata: Option<bool>,
    ) -> Result<(), VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "update_vector",
            Param6::new(collection, id, vector, metadata, namespace, merge_metadata),
            |params| params.invoke(Impl::update_vector),
        )
    }

    #[doc = "Delete vectors by IDs"]
    fn delete_vectors(
        collection: String,
        ids: Vec<Id>,
        namespace: Option<String>,
    ) -> Result<u32, VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "delete_vectors",
            Param3::new(collection, ids, namespace),
            |params| params.invoke(Impl::delete_vectors),
        )
    }

    #[doc = "Delete vectors by filter"]
    fn delete_by_filter(
        collection: String,
        filter: FilterExpression,
        namespace: Option<String>,
    ) -> Result<u32, VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "delete_by_filter",
            Param3::new(collection, filter, namespace),
            |params| params.invoke(Impl::delete_by_filter),
        )
    }

    #[doc = "Delete all vectors in namespace"]
    fn delete_namespace(collection: String, namespace: String) -> Result<u32, VectorError> {
        write_remote_durably(
            "golem_vector::vectors",
            "delete_namespace",
            Param2::new(collection, namespace),
            |params| params.invoke(Impl::delete_namespace),
        )
    }

    #[doc = "List vectors with filtering and pagination"]
    fn list_vectors(
        collection: String,
        namespace: Option<String>,
        filter: Option<FilterExpression>,
        limit: Option<u32>,
        cursor: Option<Id>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<ListResponse, VectorError> {
        read_remote_durably(
            "golem_vector::vectors",
            "list_vectors",
            Param7::new(
                collection,
                namespace,
                filter,
                limit,
                cursor,
                include_vectors,
                include_metadata,
            ),
            |params| params.invoke(Impl::list_vectors),
        )
    }

    #[doc = "Count vectors matching filter"]
    fn count_vectors(
        collection: String,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
    ) -> Result<u64, VectorError> {
        read_remote_durably(
            "golem_vector::vectors",
            "count_vectors",
            Param3::new(collection, filter, namespace),
            |params| params.invoke(Impl::count_vectors),
        )
    }
}

impl<Impl: CollectionsGuest> CollectionsGuest for DurableVector<Impl> {
    #[doc = "Create or update collection (upsert)"]
    fn upsert_collection(
        name: String,
        description: Option<String>,
        dimension: u32,
        metric: DistanceMetric,
        index_config: Option<IndexConfig>,
        metadata: Option<Metadata>,
    ) -> Result<CollectionInfo, VectorError> {
        write_remote_durably_with_side_effects(
            "golem_vector::collections",
            "upsert_collection",
            Param6::new(name, description, dimension, metric, index_config, metadata),
            |params| params.invoke(Impl::upsert_collection),
        )
    }

    #[doc = "List all collections"]
    fn list_collections() -> Result<Vec<String>, VectorError> {
        read_remote_durably(
            "golem_vector::collections",
            "list_collections",
            NoParam,
            |params| params.invoke(Impl::list_collections),
        )
    }

    #[doc = "Get collection information"]
    fn get_collection(name: String) -> Result<CollectionInfo, VectorError> {
        read_remote_durably(
            "golem_vector::collections",
            "get_collection",
            Param::new(name),
            |params| params.invoke(Impl::get_collection),
        )
    }

    #[doc = "Update collection metadata only"]
    fn update_collection(
        name: String,
        description: Option<String>,
        metadata: Option<Metadata>,
    ) -> Result<CollectionInfo, VectorError> {
        write_remote_durably(
            "golem_vector::collections",
            "update_collection",
            Param3::new(name, description, metadata),
            |params| params.invoke(Impl::update_collection),
        )
    }

    #[doc = "Delete collection and all vectors"]
    fn delete_collection(name: String) -> Result<(), VectorError> {
        write_remote_durably(
            "golem_vector::collections",
            "delete_collection",
            Param::new(name),
            |params| params.invoke(Impl::delete_collection),
        )
    }

    #[doc = "Check if collection exists"]
    fn collection_exists(name: String) -> Result<bool, VectorError> {
        read_remote_durably(
            "golem_vector::collections",
            "collection_exists",
            Param::new(name),
            |params| params.invoke(Impl::collection_exists),
        )
    }
}
