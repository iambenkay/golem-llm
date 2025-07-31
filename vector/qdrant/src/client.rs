use golem_vector::{
    error::from_reqwest_error,
    vector::{collections, search, search_extended, types, vectors},
};
use serde_json::{json, Map};

use crate::conversions::{
    from_filter_expression_to_qdrant_filter, from_id_to_json_value, from_json_value_to_id,
    from_qdrant_point_to_vector_record, from_qdrant_vector_to_vector_data,
    from_recommendation_example_to_qdrant_query, from_recommendation_strategy_to_qdrant_strategy,
    from_search_query_to_qdrant_query, from_vector_data_to_qdrant_vector,
    from_vector_record_to_qdrant_point, get_response_body, json_map_to_metadata, metadata_to_json,
    metadata_to_json_map, to_create_collection_body,
};

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
                reqwest::header::HeaderValue::from_str(&api_key).map_err(|_| {
                    types::VectorError::InvalidParams(format!(
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
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        let response = self
            .client
            .get(format!("{}/collections/{name}", self.endpoint))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<CollectionInformation> = get_response_body(response)?;

        Ok(response.to_collection_info(name))
    }

    pub fn delete_collection(&self, name: String) -> Result<(), types::VectorError> {
        let response = self
            .client
            .delete(format!("{}/collections/{name}", self.endpoint))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<bool> = get_response_body(response)?;

        if response.result {
            Ok(())
        } else {
            Err(types::VectorError::ProviderError(
                "The Qdrant collection could not be deleted".to_string(),
            ))
        }
    }

    pub fn collection_exists(&self, name: String) -> Result<bool, types::VectorError> {
        let response = self
            .client
            .get(format!("{}/collections/{name}/exists", self.endpoint))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<CollectionExists> = get_response_body(response)?;
        Ok(response.result.exists)
    }

    pub fn create_collection(
        &self,
        name: String,
        dimension: u32,
        metric: types::DistanceMetric,
        index_config: Option<collections::IndexConfig>,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        let body = to_create_collection_body(dimension, metric, index_config, metadata)?;

        let response = self
            .client
            .put(format!("{}/collections/{name}?wait=true", self.endpoint))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<bool> = get_response_body(response)?;
        if response.result {
            self.get_collection(name)
        } else {
            Err(types::VectorError::ProviderError(
                "The Qdrant collection could not be created".to_string(),
            ))
        }
    }

    pub fn update_collection(
        &self,
        name: String,
        metadata: Option<types::Metadata>,
    ) -> Result<collections::CollectionInfo, types::VectorError> {
        let body = metadata
            .map(|map| {
                map.into_iter().fold(
                    Map::<String, serde_json::Value>::new(),
                    |mut acc, (k, v)| {
                        acc.insert(k, metadata_to_json(&v));
                        acc
                    },
                )
            })
            .map(|m| json!(m))
            .unwrap_or(json!({}));

        let response = self
            .client
            .patch(format!("{}/collections/{name}", self.endpoint))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<bool> = get_response_body(response)?;
        if response.result {
            self.get_collection(name)
        } else {
            Err(types::VectorError::ProviderError(
                "The Qdrant collection could not be updated".to_string(),
            ))
        }
    }

    pub fn upsert_points(
        &self,
        collection: String,
        vectors: Vec<types::VectorRecord>,
    ) -> Result<vectors::BatchResult, types::VectorError> {
        let mut points = vec![];

        for vector in vectors {
            points.push(from_vector_record_to_qdrant_point(vector)?)
        }

        let body = json!({
            "points": points
        });

        self.client
            .put(format!(
                "{}/collections/{collection}/points?wait=true",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        Ok(vectors::BatchResult {
            success_count: 0,
            failure_count: 0,
            errors: vec![],
        })
    }

    pub fn retrieve_points(
        &self,
        collection: String,
        ids: Vec<types::Id>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::VectorRecord>, types::VectorError> {
        let ids: Vec<serde_json::Value> = ids.into_iter().map(from_id_to_json_value).collect();

        let body = json!({
            "ids": ids,
            "with_payload": include_metadata.unwrap_or_default(),
            "with_vector": include_vectors.unwrap_or_default(),
        });

        let response = self
            .client
            .post(format!("{}/collections/{collection}/points", self.endpoint))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<Vec<QdrantPoint>> = get_response_body(response)?;

        response
            .result
            .into_iter()
            .map(from_qdrant_point_to_vector_record)
            .collect()
    }

    pub fn scroll_points(
        &self,
        collection: String,
        filter: Option<types::FilterExpression>,
        limit: Option<u32>,
        cursor: Option<types::Id>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<vectors::ListResponse, types::VectorError> {
        let mut body = json!({
            "limit": limit,
            "offset": cursor.map(from_id_to_json_value),
            "with_payload": include_metadata.unwrap_or_default(),
            "with_vector": include_vectors.unwrap_or_default(),
        });

        if let Some(expr) = filter {
            let map = body.as_object_mut().unwrap();
            map["filter"] = from_filter_expression_to_qdrant_filter(&expr)?;
        }

        let response = self
            .client
            .post(format!(
                "{}/collections/{collection}/points/scroll",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let QdrantResponse { result: scroll, .. }: QdrantResponse<Scroll> =
            get_response_body(response)?;

        let mut vectors = vec![];

        for point in scroll.points {
            vectors.push(from_qdrant_point_to_vector_record(point)?);
        }

        Ok(vectors::ListResponse {
            vectors,
            total_count: None,
            next_cursor: if let Some(offset) = scroll.next_page_offset {
                Some(from_json_value_to_id(offset)?)
            } else {
                None
            },
        })
    }

    pub fn count_points(
        &self,
        collection: String,
        filter: Option<types::FilterExpression>,
    ) -> Result<u64, types::VectorError> {
        let mut request = self.client.post(format!(
            "{}/collections/{collection}/points/count",
            self.endpoint
        ));

        if let Some(expr) = filter {
            let body = json!({
                "filter": from_filter_expression_to_qdrant_filter(&expr)?
            });
            request = request.json(&body);
        }

        let response = request.send().map_err(from_reqwest_error)?;

        let response: QdrantResponse<PointCount> = get_response_body(response)?;

        Ok(response.result.count)
    }

    pub fn retrieve_point(
        &self,
        collection: String,
        id: types::Id,
    ) -> Result<Option<types::VectorRecord>, types::VectorError> {
        let id = match id {
            types::Id::Uint(uint) => format!("{uint}"),
            types::Id::Str(str) => str,
        };

        let response = self
            .client
            .get(format!(
                "{}/collections/{collection}/points/{id}",
                self.endpoint
            ))
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<QdrantPoint> = get_response_body(response)?;

        from_qdrant_point_to_vector_record(response.result).map(Some)
    }

    pub fn delete_points(
        &self,
        collection: String,
        ids: Option<Vec<types::Id>>,
        filter: Option<types::FilterExpression>,
    ) -> Result<u32, types::VectorError> {
        let ids: Option<Vec<serde_json::Value>> =
            ids.map(|ids| ids.into_iter().map(from_id_to_json_value).collect());

        let body = if let Some(ids) = ids {
            json!({
                "points": ids,
            })
        } else if let Some(expr) = filter {
            json!({
                "filter": from_filter_expression_to_qdrant_filter(&expr)?,
            })
        } else {
            return Err(types::VectorError::InvalidParams(
                "You must provide either a list of ids or a filter to delete points".to_string(),
            ));
        };

        self.client
            .post(format!(
                "{}/collections/{collection}/points/delete?wait=true",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        Ok(0)
    }

    pub fn update_vectors(
        &self,
        collection: String,
        id: types::Id,
        vector: Option<types::VectorData>,
        metadata: Option<types::Metadata>,
        merge_metadata: Option<bool>,
    ) -> Result<(), types::VectorError> {
        let id = from_id_to_json_value(id);

        let mut body = vec![];

        if let Some(vector) = vector {
            body.push(json!({
                "update_vectors": {
                    "points": [
                        {
                            "id": id,
                            "vector": from_vector_data_to_qdrant_vector(vector)?
                        }
                    ]
                }
            }));
        }

        if let Some(metadata) = metadata {
            let key = if merge_metadata.unwrap_or_default() {
                "set_payload"
            } else {
                "overwrite_payload"
            };

            body.push(json!({
                key: {
                    "points": [id],
                    "payload": metadata_to_json_map(metadata)
                }
            }));
        }

        self.client
            .post(format!(
                "{}/collections/{collection}/points/batch?wait=true",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        Ok(())
    }

    pub fn search_points(
        &self,
        collection: String,
        query: search::SearchQuery,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let params = search_params.map(|param| {
            param
                .iter()
                .fold(serde_json::Map::new(), |mut acc, (key, value)| {
                    acc.insert(key.clone(), json!(value));
                    acc
                })
        });

        let query = from_search_query_to_qdrant_query(query)?;

        let mut body = json!({
            "limit": limit,
            "with_vector": include_vectors.unwrap_or_default(),
            "with_payload": include_metadata.unwrap_or_default(),
            "score_threshold": min_score,
            "using": namespace,
            "params": params,
            "query": query,
        });

        if let Some(filter) = filter {
            let body = body.as_object_mut().unwrap();
            body["filter"] = from_filter_expression_to_qdrant_filter(&filter)?;
        }

        self.query_points(collection, body)
    }

    pub fn recommend_points(
        &self,
        collection: String,
        positive: Vec<search_extended::RecommendationExample>,
        negative: Option<Vec<search_extended::RecommendationExample>>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        strategy: Option<search_extended::RecommendationStrategy>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let positive = positive.into_iter().try_fold(vec![], |mut acc, rec| {
            acc.push(from_recommendation_example_to_qdrant_query(rec)?);
            Ok(acc)
        })?;

        let negative = if let Some(negative) = negative {
            Some(negative.into_iter().try_fold(vec![], |mut acc, rec| {
                acc.push(from_recommendation_example_to_qdrant_query(rec)?);
                Ok(acc)
            })?)
        } else {
            None
        };

        let query = json!({
            "recommend": {
                "positive": positive,
                "negative": negative,
                "strategy": strategy.map(from_recommendation_strategy_to_qdrant_strategy),
            }
        });

        let mut body = json!({
            "limit": limit,
            "with_vector": include_vectors.unwrap_or_default(),
            "with_payload": include_metadata.unwrap_or_default(),
            "using": namespace,
            "query": query,
        });

        if let Some(filter) = filter {
            let body = body.as_object_mut().unwrap();
            body["filter"] = from_filter_expression_to_qdrant_filter(&filter)?;
        }

        self.query_points(collection, body)
    }

    pub fn discover_points(
        &self,
        collection: String,
        context_pairs: Vec<search_extended::ContextPair>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let contexts = context_pairs
            .into_iter()
            .try_fold(vec![], |mut acc, context| {
                acc.push(json!({
                    "positive": from_recommendation_example_to_qdrant_query(context.positive)?,
                    "negative": from_recommendation_example_to_qdrant_query(context.negative)?,
                }));
                Ok(acc)
            })?;

        let query = json!({
            "discover": contexts
        });

        let mut body = json!({
            "limit": limit,
            "with_vector": include_vectors.unwrap_or_default(),
            "with_payload": include_metadata.unwrap_or_default(),
            "using": namespace,
            "query": query,
        });

        if let Some(filter) = filter {
            let body = body.as_object_mut().unwrap();
            body["filter"] = from_filter_expression_to_qdrant_filter(&filter)?;
        }

        self.query_points(collection, body)
    }

    fn query_points(
        &self,
        collection: String,
        body: serde_json::Value,
    ) -> Result<Vec<types::SearchResult>, types::VectorError> {
        let response = self
            .client
            .post(format!(
                "{}/collections/{collection}/points/query",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<PointSearchResult> = get_response_body(response)?;

        response
            .result
            .points
            .into_iter()
            .try_fold(vec![], |mut acc, point| {
                acc.push(types::SearchResult {
                    id: from_json_value_to_id(point.id)?,
                    score: point.score as f32,
                    distance: None,
                    vector: from_qdrant_vector_to_vector_data(point.vector).ok(),
                    metadata: point
                        .payload
                        .and_then(|payload| Some(json_map_to_metadata(payload.as_object()?))),
                });
                Ok(acc)
            })
    }

    pub fn query_point_groups(
        &self,
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
        let query = from_search_query_to_qdrant_query(query)?;

        let mut body = json!({
            "limit": max_groups,
            "with_vector": include_vectors.unwrap_or_default(),
            "with_payload": include_metadata.unwrap_or_default(),
            "using": namespace,
            "query": query,
            "group_size": group_size,
            "group_by": group_by,
        });

        if let Some(filter) = filter {
            let body = body.as_object_mut().unwrap();
            body["filter"] = from_filter_expression_to_qdrant_filter(&filter)?;
        }

        let response = self
            .client
            .post(format!(
                "{}/collections/{collection}/points/query/groups",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<GroupPointResult> = get_response_body(response)?;

        response
            .result
            .groups
            .into_iter()
            .try_fold(vec![], |mut acc, group| {
                acc.push(search_extended::GroupedSearchResult {
                    group_count: group.hits.len() as u32,
                    group_value: types::MetadataValue::StringVal(group.id),
                    results: group.hits.into_iter().try_fold(vec![], |mut acc, hit| {
                        acc.push(types::SearchResult {
                            id: from_json_value_to_id(hit.id)?,
                            score: hit.score as f32,
                            distance: None,
                            vector: from_qdrant_vector_to_vector_data(hit.vector).ok(),
                            metadata: hit.payload.and_then(|payload| {
                                Some(json_map_to_metadata(payload.as_object()?))
                            }),
                        });
                        Ok(acc)
                    })?,
                });
                Ok(acc)
            })
    }

    pub fn batch_query_points(
        &self,
        collection: String,
        queries: Vec<search::SearchQuery>,
        limit: u32,
        filter: Option<types::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<Vec<types::SearchResult>>, types::VectorError> {
        let params = search_params.map(|param| {
            param
                .iter()
                .fold(serde_json::Map::new(), |mut acc, (key, value)| {
                    acc.insert(key.clone(), json!(value));
                    acc
                })
        });

        let mut searches = vec![];

        for query in queries {
            let query = from_search_query_to_qdrant_query(query)?;

            let mut body = json!({
                "limit": limit,
                "with_vector": include_vectors.unwrap_or_default(),
                "with_payload": include_metadata.unwrap_or_default(),
                "score_threshold": min_score,
                "params": params,
                "using": namespace,
                "query": query,
            });
            if let Some(ref filter) = filter {
                let body = body.as_object_mut().unwrap();
                body["filter"] = from_filter_expression_to_qdrant_filter(filter)?;
            }

            searches.push(body);
        }

        let body = json!({
            "searches": searches,
        });

        let response = self
            .client
            .post(format!(
                "{}/collections/{collection}/points/query/batch",
                self.endpoint
            ))
            .json(&body)
            .send()
            .map_err(from_reqwest_error)?;

        let response: QdrantResponse<Vec<PointSearchResult>> = get_response_body(response)?;

        let mut results = vec![];

        for result in response.result {
            results.push(
                result
                    .points
                    .into_iter()
                    .try_fold(vec![], |mut acc, point| {
                        acc.push(types::SearchResult {
                            id: from_json_value_to_id(point.id)?,
                            score: point.score as f32,
                            distance: None,
                            vector: from_qdrant_vector_to_vector_data(point.vector).ok(),
                            metadata: point
                                .payload
                                .and_then(|point| Some(json_map_to_metadata(point.as_object()?))),
                        });
                        Ok(acc)
                    })?,
            )
        }

        Ok(results)
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
struct PointSearchResult {
    points: Vec<SearchEntry>,
}

#[derive(serde::Deserialize)]
struct GroupPointResult {
    groups: Vec<Group>,
}

#[derive(serde::Deserialize)]
struct Group {
    id: String,
    hits: Vec<SearchEntry>,
}

#[derive(serde::Deserialize)]
pub struct SearchEntry {
    pub id: serde_json::Value,
    #[serde(rename = "version")]
    pub _version: u64,
    pub score: f64,
    pub payload: Option<serde_json::Value>,
    pub vector: serde_json::Value,
}

impl QdrantUsage {
    fn to_provider_stats(&self) -> types::Metadata {
        vec![
            (
                "cpu".to_string(),
                types::MetadataValue::IntegerVal(self.cpu as i64),
            ),
            (
                "payload_io_read".to_string(),
                types::MetadataValue::IntegerVal(self.payload_io_read as i64),
            ),
            (
                "payload_io_write".to_string(),
                types::MetadataValue::IntegerVal(self.payload_io_write as i64),
            ),
            (
                "payload_index_io_read".to_string(),
                types::MetadataValue::IntegerVal(self.payload_index_io_read as i64),
            ),
            (
                "payload_index_io_write".to_string(),
                types::MetadataValue::IntegerVal(self.payload_index_io_write as i64),
            ),
            (
                "vector_io_read".to_string(),
                types::MetadataValue::IntegerVal(self.vector_io_read as i64),
            ),
            (
                "vector_io_write".to_string(),
                types::MetadataValue::IntegerVal(self.vector_io_write as i64),
            ),
        ]
    }
}

#[derive(serde::Deserialize)]
struct CollectionName {
    name: String,
}

#[derive(serde::Deserialize)]
struct PointCount {
    count: u64,
}

#[derive(serde::Deserialize)]
struct CollectionExists {
    exists: bool,
}

#[derive(serde::Deserialize)]
struct CollectionInformation {
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "optimizer_status")]
    _optimizer_status: String,
    #[serde(rename = "segments_count")]
    _segments_count: u64,
    #[serde(rename = "points_count")]
    _points_count: u64,
    vectors_count: u64,
    indexed_vectors_count: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct QdrantPoint {
    pub id: serde_json::Value,
    pub payload: Option<serde_json::Value>,
    pub vector: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct Scroll {
    points: Vec<QdrantPoint>,
    next_page_offset: Option<serde_json::Value>,
}

impl QdrantResponse<CollectionInformation> {
    fn to_collection_info(&self, name: String) -> collections::CollectionInfo {
        collections::CollectionInfo {
            name,
            description: None,
            dimension: None,
            metric: None,
            vector_count: self.result.vectors_count,
            size_bytes: None,
            index_ready: self.result.indexed_vectors_count > 0,
            created_at: None,
            updated_at: None,
            provider_stats: Some(self.usage.to_provider_stats()),
        }
    }
}
